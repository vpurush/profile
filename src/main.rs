#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use profile::app::*;
    use profile::fileserv::file_and_error_handler;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    // We don't use an address for the lambda function
    #[allow(unused_variables)]
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);
    // In development, we use the Hyper server
    #[cfg(debug_assertions)]
    {
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        logging::log!("listening on http://{}", &addr);
        axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    }

    // In release, we use the lambda_http crate
    #[cfg(not(debug_assertions))]
    {
        let app = tower::ServiceBuilder::new()
            .layer(axum_aws_lambda::LambdaLayer::default())
            .service(app);

        lambda_http::run(app).await.unwrap();
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}

// use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

// /// This is the main body for the function.
// /// Write your code inside it.
// /// There are some code examples in the Runtime repository:
// /// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
// async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
//     // Extract some useful information from the request
//     // logging::log!("_event {}", _event);
//     println!("_event {:?}", _event);

//     // Return something that implements IntoResponse.
//     // It will be serialized to the right response event automatically by the runtime
//     let resp = Response::builder()
//         .status(200)
//         .header("content-type", "text/html")
//         .body("Hello from AWS Lambda HTTP request".into())
//         .map_err(Box::new)?;
//     Ok(resp)
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     // required to enable CloudWatch error logging by the runtime
//     tracing::init_default_subscriber();

//     run(service_fn(function_handler)).await
// }