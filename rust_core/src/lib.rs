mod app_image;
mod image_info;

use std::io::{Cursor};
use log::{error, Level};
use log::info;

use base64::{engine::general_purpose};
use std::sync::{Mutex};
use base64::{Engine};
use image::{ImageFormat, imageops};
use image::imageops::{FilterType};
use wasm_bindgen::prelude::*;
use regex::{Regex};
use crate::app_image::{AppImage, MimeType};
use crate::image_info::ImageInfo;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
}

static IMAGE_INCREMENT: Mutex<u32> = Mutex::new(0);
static IMAGES: Mutex<Vec<AppImage>> = Mutex::new(Vec::new());

#[wasm_bindgen]
pub fn initialize() {
    match console_log::init_with_level(Level::Debug) {
        Err(error) => {
            console_log(&format!("error initializing logger: {error}"))
        }
        Ok(_) => {
            info!("logger initialized")
        }
    }
}

#[wasm_bindgen]
pub fn add_image(name: &str, data_url: &str) {
    let url_parts:Vec<&str> = data_url.split("base64,").collect();
    let data_url_prefix = format!("{}base64,",url_parts[0]);
    let regex = Regex::new(r"data:(.+);base64,").unwrap();

    match regex.captures(&data_url_prefix){
        None => {
            info!("Invalid data url format: {}",data_url_prefix)
        }
        Some(captures) => {
            let group = captures.get(1).unwrap();
            let mime_type: String = group.as_str().to_string();

            let image_base64 = url_parts[1];
            let image_bytes_result = general_purpose::STANDARD.decode(image_base64);
            match image_bytes_result {
                Err(error) => {
                    error!("failed to decode image: {error}")
                }
                Ok(image_bytes) => {
                    let image = image::load_from_memory(&image_bytes).unwrap();
                    let mut images_mutex = IMAGES.lock().unwrap();
                    let mut image_id_mutex = IMAGE_INCREMENT.lock().unwrap();
                    images_mutex.push(AppImage{
                        id: *image_id_mutex,
                        name: name.to_string(),
                        width: image.width(),
                        height: image.height(),
                        mime_type: match mime_type.as_str(){
                            "image/png"=>MimeType::ImagePng,
                            "image/jpeg"=>MimeType::ImageJpeg,
                            "image/webp"=>MimeType::ImageWebP,
                            _ => {
                                error!("Invalid mime type. Only png and jpeg is supported currently.");
                                return;
                            }
                        },
                        lib_image: image
                    });
                    *image_id_mutex +=1
                }
            }
        }
    }

}

#[wasm_bindgen]
pub fn get_images_info()-> String{
    let mut images_info = Vec::new();
    for app_image in IMAGES.lock().unwrap().iter(){
        images_info.push(ImageInfo::from(app_image))
    }

    return serde_json::to_string(&images_info).unwrap();
}

#[wasm_bindgen]
pub fn scale_image(id: u32, width: u32, height: u32, mime_type: &str, smooth: bool, custom_filter: &str) -> String{
    let images_mutex = &*IMAGES.lock().unwrap();
    let image_option= images_mutex.iter().find(|e|e.id == id);
    match image_option {
        None => {
            info!("No image to scale");
        }
        Some(app_image) => {
            let image = &app_image.lib_image;
            let image = imageops::resize(image, width, height, match custom_filter{
                "Triangle"=>FilterType::Triangle,
                "CatmullRom"=>FilterType::CatmullRom,
                "Lanczos3"=>FilterType::Lanczos3,
                _=>match smooth {false=>FilterType::Nearest, true=>FilterType::Gaussian}
            });
            let mut buffer = Vec::new();
            match mime_type{
                "image/png" => {
                    let _ = image.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png);
                }
                "image/jpeg" => {
                    let image = image::DynamicImage::from(image).into_rgb8();
                    let _ = image.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Jpeg);
                }
                "image/webp" => {
                    let _ = image.write_to(&mut Cursor::new(&mut buffer), ImageFormat::WebP);
                }
                _=>{
                    info!("Not compatible format: {mime_type}")
                }
            }
            let image_base64 = general_purpose::STANDARD.encode(&buffer);
            return format!("data:{mime_type};base64,{image_base64}");
        }
    }
    "".to_string()
}
//
// pub fn set_image_callback(image_callback: &js_sys::Function) {
//     let mut current_callback = IMAGE_CALLBACK.lock().unwrap();
//     *current_callback = Some(image_callback);
// }


