mod utils;

use apng;
use apng::Encoder;
use apng::{Frame, PNGImage};
use std::io::BufWriter;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
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

#[wasm_bindgen (js_name = apngEncode)]
pub fn apng_encode(data: Vec<u8>) {
    let img = image::load_from_memory_with_format(&data, image::ImageFormat::PNG).unwrap();

    let mut png_images: Vec<PNGImage> = Vec::new();
    png_images.push(apng::load_dynamic_image(img).unwrap());

    let mut buf_writer = BufWriter::new(Vec::new());

    let config = apng::create_config(&png_images, None).unwrap();
    let mut encoder = Encoder::new(&mut buf_writer, config).unwrap();
    let frame = Frame {
        delay_num: Some(1),
        delay_den: Some(2),
        ..Default::default()
    };

    match encoder.encode_all(png_images, Some(&frame)) {
        Ok(_n) => log("success"),
        Err(err) => console_log!("{}", err),
    }

    let buffer = buf_writer.is_empty();
    console_log!("{}", buffer);
}
