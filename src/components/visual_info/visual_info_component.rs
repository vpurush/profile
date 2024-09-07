use crate::components::content_panel::types::{ContentPanel, ContentfulContentPanel};
use crate::components::panel::types::ContentfulPanel;
use crate::contentful_richtext_renderer::renderer::render_document_to_html;
use leptos::ServerFnError::ServerError;
use leptos::{component, create_resource, server, view, Children, IntoView, ServerFnError, SignalGet, Suspense};
use crate::components::visual_info::types::{ContentfulVisualInfoTextAlign, VisualInfo};

#[component]
pub fn VisualInfoComponent(visual_info: VisualInfo, children: Children) -> impl IntoView {
    let get_classes = move || {
        let mut classes = vec!["visual-info"];
        let text_align_class = match visual_info.text_alignment {
            ContentfulVisualInfoTextAlign::Left => "left-align",
            ContentfulVisualInfoTextAlign::Right => "right-align",
            ContentfulVisualInfoTextAlign::Center => "center-align",
            ContentfulVisualInfoTextAlign::Justified => "justified-align"
        };
        classes.push(text_align_class);
        classes.join(" ")
    };

    view! {
        <div class={get_classes}>{children()}</div>
    }
}
