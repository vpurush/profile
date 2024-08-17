use leptos::{component, create_resource, server, view, IntoView, ServerFnError, SignalGet, Suspense};
use leptos::ServerFnError::ServerError;
use crate::components::page::get_page;
use crate::components::page::get_page::{get_page, ContentfulPage};

#[server(prefix = "/api")]
pub async fn get_page_by_slug(slug: String) -> Result<ContentfulPage, ServerFnError> {
    match get_page(slug).await {
        Ok(pageCollection) => Ok(pageCollection.data.page_collection.items.into_iter().next().unwrap()),
        Err(error) =>  {
            println!("Server error {}", error);
            Err(ServerFnError::ServerError(error.to_string()))
        }
    }
    // Ok("This is a Page Component that fetches content from server.".to_string())
}

#[component]
pub fn PageComponent() -> impl IntoView {
    let content_resource =
        create_resource(|| (), |_| async move { get_page_by_slug("/".to_string()).await });
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
