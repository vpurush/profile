use crate::components::page::get_page;
use crate::components::page::get_page::{get_page, ContentfulPage};
use leptos::ServerFnError::ServerError;
use leptos::{
    component, create_resource, server, view, IntoView, ServerFnError, SignalGet, Suspense,
};

#[server(prefix = "/api")]
pub async fn get_page_by_slug(slug: String) -> Result<ContentfulPage, ServerFnError> {
    match get_page(slug).await {
        Ok(page_collection_response) => match (page_collection_response
            .page_collection
            .items
            .into_iter()
            .next()) {
            Option::Some(page) => Ok(page),
            Option::None => Err(ServerFnError::ServerError("Not found".to_string()))
        },
        Err(error) => {
            println!("Server error {}", error);
            Err(ServerFnError::ServerError(error.to_string()))
        }
    }
    // Ok("This is a Page Component that fetches content from server.".to_string())
}

#[component]
pub fn PageComponent() -> impl IntoView {
    let content_resource = create_resource(
        || (),
        |_| async move {
            println!(
                "path {:?} {}",
                leptos_router::use_route().path(),
                leptos_router::use_location().pathname.get()
            );
            let path = leptos_router::use_location().pathname.get();
            get_page_by_slug(path).await
        },
    );
    // let content = get_page("/".to_string());
    // content.

    // content_resource.

    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p>}>
        {move || {
            content_resource.get().map(|data| {
                view! {
                    <div>{format!("{:?}", data)}</div>
                }
            })
        }}
        </Suspense>
    }
}
