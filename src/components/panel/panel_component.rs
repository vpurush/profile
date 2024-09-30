use crate::components::content_media_panel::content_media_panel_component::ContentMediaPanelComponent;
use crate::components::content_panel::content_panel_component::ContentPanelComponent;
use crate::components::panel::types::Panel::ContentMediaPanel;
use crate::components::panel::types::{ContentfulPanel, Panel};
use crate::components::picture_panel::picture_panel_component::PicturePanelComponent;
use crate::components::visual_info::visual_info_component::VisualInfoComponent;
use leptos::ServerFnError::ServerError;
use leptos::{
    component, create_resource, server, view, IntoView, ServerFnError, SignalGet, Suspense,
};

#[component]
pub fn PanelComponent(panel: Panel) -> impl IntoView {
    match panel {
        Panel::ContentPanel(content_panel) => view! {
            <VisualInfoComponent visual_info={content_panel.visual_info.clone()}>
                <ContentPanelComponent content_panel=content_panel />
            </VisualInfoComponent>
        },
        Panel::PicturePanel(picture_panel) => view! {
            <VisualInfoComponent visual_info={picture_panel.visual_info.clone()}>
                <PicturePanelComponent picture_panel=picture_panel />
            </VisualInfoComponent>
        },
        Panel::ContentMediaPanel(content_media_panel) => view! {
           <VisualInfoComponent visual_info={content_media_panel.visual_info.clone()}>
               <ContentMediaPanelComponent content_media_panel=content_media_panel />
           </VisualInfoComponent>
        },
    }
}
