mod utils;

use apng;
use apng::Encoder;
use apng::{Frame, PNGImage};
use serde::{Deserialize, Serialize};
use std::io::BufWriter;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn initialize() {
    utils::set_panic_hook();
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Example {
    pub field: Vec<Vec<u8>>,
    pub field2: Vec<u8>,
}

#[wasm_bindgen (js_name = apngInit)]
pub fn apng_init() -> JsValue {
    let example = Example {
        field: vec![vec![]],
        field2: vec![],
    };

    JsValue::from_serde(&example).unwrap()
}

#[wasm_bindgen (js_name = apngEncodeAll)]
pub fn apng_encode_all(data: &JsValue) {
    console_log!("encoding in apng_encode");

    console_log!("{:?}", data);

    let example: Example = data.into_serde().unwrap();

    console_log!("{:?}", example)
}

#[wasm_bindgen (js_name = apngEncode)]
pub fn apng_encode(data: Vec<u8>, data2: Vec<u8>, data3: Vec<u8>) -> Vec<u8> {
    let img = image::load_from_memory_with_format(&data, image::ImageFormat::PNG).unwrap();
    let img2 = image::load_from_memory_with_format(&data2, image::ImageFormat::PNG).unwrap();
    let img3 = image::load_from_memory_with_format(&data3, image::ImageFormat::PNG).unwrap();

    let mut png_images: Vec<PNGImage> = Vec::new();
    png_images.push(apng::load_dynamic_image(img).unwrap());
    png_images.push(apng::load_dynamic_image(img2).unwrap());
    png_images.push(apng::load_dynamic_image(img3).unwrap());

    let mut buf = Vec::new();
    {
        let mut buf_writer = BufWriter::new(&mut buf);

        let config = apng::create_config(&png_images, None).unwrap();
        let mut encoder = Encoder::new(&mut buf_writer, config).unwrap();
        let frame = Frame {
            delay_num: Some(1),
            delay_den: Some(2),
            ..Default::default()
        };

        match encoder.encode_all(png_images, Some(&frame)) {
            Ok(_n) => log("success apng encode!!!"),
            Err(err) => console_log!("{}", err),
        }
    }

    buf
}
