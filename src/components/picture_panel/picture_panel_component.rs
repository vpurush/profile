use leptos::{component, view, IntoView};
use crate::components::picture_panel::types::PicturePanel;

#[component]
pub fn PicturePanelComponent(picture_panel: PicturePanel) -> impl IntoView {
    view! {
        <div class="picture-panel">
            <img class="image" src={picture_panel.image.url} />
        </div>
    }
}
