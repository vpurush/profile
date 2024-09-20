use leptos::{component, view, IntoView};
use crate::components::picture_panel::types::PicturePanel;

#[component]
pub fn PicturePanelComponent(picture_panel: PicturePanel) -> impl IntoView {
    view! {
        <img src={picture_panel.image.url} />
    }
}
