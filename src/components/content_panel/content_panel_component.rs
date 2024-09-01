use crate::components::content_panel::types::ContentPanel;
use crate::components::panel::types::Panel;
use leptos::ServerFnError::ServerError;
use leptos::{
    component, create_resource, server, view, IntoView, ServerFnError, SignalGet, Suspense,
};

#[component]
pub fn ContentPanelComponent(content_panel: ContentPanel) -> impl IntoView {
    view! {
        <div>This is ContentPanel</div>
    }
}
