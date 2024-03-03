use image::{DynamicImage};
use serde::Serialize;

pub struct AppImage{
    pub id: u32,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub mime_type: MimeType,
    pub lib_image: DynamicImage
}

// impl AppImage{
//     pub fn base64(&self) -> String{
//         let mut buffer = Vec::new();
//         match self.mime_type{
//             MimeType::ImagePng => {
//                 let _ = self.lib_image.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png);
//             }
//             MimeType::ImageJpeg => {
//                 // TODO: fix bug exporting Jpeg
//                 let _ = self.lib_image.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png);
//             }
//         }
//         return general_purpose::STANDARD.encode(&buffer);
//     }
// }

#[derive(Clone, Serialize)]
pub enum MimeType{
    ImagePng,
    ImageJpeg,
}
impl std::fmt::Display for MimeType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", match self {
            MimeType::ImagePng => {"image/png"}
            MimeType::ImageJpeg => {"image/jpeg"}
        })
    }
}