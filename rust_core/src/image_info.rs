use serde::Serialize;
use crate::app_image::{AppImage};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageInfo {
    pub id: u32,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub mime_type: String,
}

impl ImageInfo {
    pub fn from(app_image: &AppImage) -> ImageInfo {
        ImageInfo {
            id: app_image.id,
            name: app_image.name.to_string(),
            width: app_image.width,
            height: app_image.height,
            mime_type: format!("{}",app_image.mime_type)
        }
    }
}