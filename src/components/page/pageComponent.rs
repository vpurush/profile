use leptos::{component, create_resource, server, view, IntoView, ServerFnError, SignalGet, Suspense};

#[server(prefix = "/api")]
pub async fn get_page(slug: String) -> Result<String, ServerFnError> {
    Ok("This is a Page Component that fetches content from server.".to_string())
}

#[component]
pub fn PageComponent() -> impl IntoView {
    let content_resource =
        create_resource(|| (), |_| async move { get_page("/".to_string()).await });
    // let content = get_page("/".to_string());
    // content.

    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p>}>
        {move || {
            content_resource.get().map(|data| {
                view! {
                    <div>{data}</div>
                }
            })
        }}
        </Suspense>
    }
}
