use leptos::ServerFnError::ServerError;
use leptos::{
    component, create_resource, server, view, IntoView, ServerFnError, SignalGet, Suspense,
};
use crate::components::content_panel::content_panel_component::ContentPanelComponent;
use crate::components::panel::types::ContentfulPanel;

#[component]
pub fn PanelComponent(panel: ContentfulPanel) -> impl IntoView {
    match panel {
        ContentfulPanel::ContentPanel(content_panel) => view! {
            <ContentPanelComponent content_panel=content_panel />
        }
    }
}
