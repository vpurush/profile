use leptos::ServerFnError::ServerError;
use leptos::{
    component, create_resource, server, view, IntoView, ServerFnError, SignalGet, Suspense,
};
use crate::components::content_panel::content_panel_component::ContentPanelComponent;
use crate::components::panel::types::{ContentfulPanel, Panel};

#[component]
pub fn PanelComponent(panel: Panel) -> impl IntoView {
    match panel {
        Panel::ContentPanel(content_panel) => view! {
            <ContentPanelComponent content_panel=content_panel />
        }
    }
}
