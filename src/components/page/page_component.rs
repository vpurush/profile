use crate::common::application_error::ApplicationError;
use crate::components::page::get_page;
use crate::components::page::get_page::get_page_collection;
use crate::components::page::types::{ContentfulPage, Page};
use crate::components::panel::panel_component::PanelComponent;
use leptos::ServerFnError::ServerError;
use leptos::{
    component, create_resource, server, view, CollectView, IntoView, ServerFnError, SignalGet,
    Suspense,
};

#[server(prefix = "/api")]
pub async fn get_page_by_slug(slug: String) -> Result<Page, ServerFnError> {
    match get_page_collection(slug).await {
        Ok(page_collection_response) => match (page_collection_response
            .page_collection
            .into_iter()
            .next())
        {
            Option::Some(page) => Ok(page),
            Option::None => Err(ServerFnError::ServerError(String::from("Not Found"))),
        },
        Err(error) => {
            println!("Server error {}", error);
            Err(ServerFnError::ServerError(error.to_string()))
        }
    }
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

    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p>}>
        {move || {
            content_resource.get().map(|page_result| {
                match page_result {
                    Ok(page) => view! {
                        <div>
                            <h1>{page.title}</h1>
                            {page.panels_collection.into_iter().map(|panel_item| {
                                view! {
                                    <PanelComponent panel=panel_item />
                                }
                            }).collect_view()}
                        </div>
                    },
                    Err(err) =>
                        match err {
                            ServerFnError::ServerError(s) => {
                                if s == "Not Found" {
                                    view! {
                                        <div>Page not found</div>
                                    }
                                } else {
                                    view! {
                                        <div> Error occurred {format!("{:?}", s)}</div>
                                    }
                                }
                            }
                            _ => {
                                view! {
                                    <div> Error occurred {format!("{:?}", err)}</div>
                                }
                            }
                        },
                }
            })
        }}
        </Suspense>
    }
}
