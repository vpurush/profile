use crate::components::picture_panel::types::{
    Asset, ContentfulAsset, ContentfulPicturePanel, PicturePanel,
};
use crate::components::visual_info::get_visual_info::visual_info_decorator;

pub fn get_asset_query() -> String {
    String::from(
        "\
      url
      width
      height
    ",
    )
}
pub fn get_picture_panel_query() -> String {
    format!(
        "\
        ... on PicturePanel{{
          title
          imageCaption
          image {{
            {}
          }}
          visualInfo {{
            textAlignment
            backgroundColor
          }}
        }}
    ",
        get_asset_query()
    )
}

pub fn asset_decorator(contentful_asset: ContentfulAsset) -> Asset {
    Asset {
        url: contentful_asset.url,
        height: contentful_asset.height,
        width: contentful_asset.width,
    }
}

pub fn picture_panel_decorator(contentful_picture_panel: ContentfulPicturePanel) -> PicturePanel {
    PicturePanel {
        title: contentful_picture_panel.title,
        image_caption: contentful_picture_panel.image_caption,
        image: asset_decorator(contentful_picture_panel.image),
        visual_info: visual_info_decorator(contentful_picture_panel.visual_info),
    }
}
