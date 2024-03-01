use std::io::{Cursor};
use log::{error, Level};
use log::info;

use base64::{engine::general_purpose};
use std::sync::{Mutex};
use base64::{Engine};
use image::{DynamicImage, ImageFormat, imageops};
use image::imageops::FilterType;
use wasm_bindgen::prelude::*;
use regex::{Regex};

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
    fn alert(s: &str);
}

static IMAGE: Mutex<Option<(String,DynamicImage)>> = Mutex::new(None);
// static IMAGE_CALLBACK: Arc<Mutex<Option<&js_sys::Function>>> = Arc::new(Mutex::new(None));

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

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
pub fn set_image(data_url: &str) {
    let url_parts:Vec<&str> = data_url.split("base64,").collect();
    let data_url_prefix = format!("{}base64,",url_parts[0]);
    let regex = Regex::new(r"data:image/(.+);base64,").unwrap();

    match regex.captures(&data_url_prefix){
        None => {
            info!("Invalid image format: {}",data_url_prefix)
        }
        Some(captures) => {
            let group = captures.get(1).unwrap();
            let mime: String = group.as_str().to_string();

            let image_base64 = url_parts[1];
            let image_bytes_result = general_purpose::STANDARD.decode(image_base64);
            match image_bytes_result {
                Err(error) => {
                    error!("failed to decode image: {error}")
                }
                Ok(image_bytes) => {
                    let image = image::load_from_memory(&image_bytes).unwrap();
                    let mut image_mutex = IMAGE.lock().unwrap();
                    *image_mutex = Some((mime,image))
                }
            }
        }
    }

}

#[wasm_bindgen]
pub fn scale_image(scale: f32, ratio: f32) -> String{
    let image_mutex = IMAGE.lock().unwrap();
    let image_option = &*image_mutex;
    match image_option {
        None => {
            info!("No image to scale");
        }
        Some((mime, image)) => {
            let image = imageops::resize(image, (scale*ratio*500.0).round() as u32, (scale*500.0).round() as u32, FilterType::Nearest);
            let mut buffer = Vec::new();
            match mime.as_str() {
                "png"=>{
                    let _ = image.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png);
                }
                "jpeg"=>{
                    let _ = image.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Jpeg);
                }
                _=>{
                    info!("Not compatible format: {mime}")
                }
            }
            let image_base64 = general_purpose::STANDARD.encode(&buffer);
            return format!("data:image/{mime};base64,{image_base64}");
        }
    }
    "".to_string()
}
//
// pub fn set_image_callback(image_callback: &js_sys::Function) {
//     let mut current_callback = IMAGE_CALLBACK.lock().unwrap();
//     *current_callback = Some(image_callback);
// }


