
use leptos::{component, create_resource, server, view, IntoView, ServerFnError, SignalGet, Suspense, View};
use crate::components::content_media_panel::types::{ContentMediaPanel, ContentMediaPanelVariants};
use crate::components::content_panel::content_panel_component::ContentPanelComponent;
use crate::components::picture_panel::picture_panel_component::PicturePanelComponent;

#[component]
pub fn ContentMediaPanelComponent(content_media_panel: ContentMediaPanel) -> impl IntoView {
    let leftComponent = match content_media_panel.variant {
        ContentMediaPanelVariants::Left => view! {
            <ContentPanelComponent content_panel={content_media_panel.content.clone()} />
        },
        ContentMediaPanelVariants::Right => view! {
            <PicturePanelComponent picture_panel={content_media_panel.media.clone()} />
        },
    };
    let rightComponent = match content_media_panel.variant {
        ContentMediaPanelVariants::Left => view! {
            <PicturePanelComponent picture_panel={content_media_panel.media.clone()} />
        },
        ContentMediaPanelVariants::Right => view! {
            <ContentPanelComponent content_panel={content_media_panel.content.clone()} />
        },
    };

    let content_media_panel_classes = format!("content-media-panel {}", content_media_panel.variant);


    view! {
        <div class={{content_media_panel_classes}}>
            <div class="left-panel">
                {{leftComponent}}
            </div>
            <div class="right-panel">
                {{rightComponent}}
            </div>
        </div>
    }
}
