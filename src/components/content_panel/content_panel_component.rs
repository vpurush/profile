use crate::components::content_panel::types::{ContentPanel, ContentfulContentPanel};
use crate::components::panel::types::ContentfulPanel;
use crate::components::visual_info::visual_info_component::VisualInfoComponent;
use crate::contentful_richtext_renderer::renderer::render_document_to_html;
use leptos::ServerFnError::ServerError;
use leptos::{
    component, create_resource, server, view, IntoView, ServerFnError, SignalGet, Suspense,
};

#[component]
pub fn ContentPanelComponent(content_panel: ContentPanel) -> impl IntoView {
    view! {
        <VisualInfoComponent visual_info={content_panel.visual_info}>
            <div inner_html=render_document_to_html(content_panel.content.json)></div>
        </VisualInfoComponent>
    }
}
