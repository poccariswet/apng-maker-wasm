mod utils;

use apng;
use apng::Encoder;
use apng::{Frame, PNGImage};
use std::io::BufWriter;
use wasm_bindgen::JsCast;

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

#[wasm_bindgen (js_name = apngEncodeAll)]
pub fn apng_encode_all(data: js_sys::Array, frame_speed: f64) -> Vec<u8> {
    let data: Vec<Vec<u8>> = data
        .values()
        .into_iter()
        .map(|x| {
            x.unwrap_throw()
                .unchecked_ref::<js_sys::Uint8Array>()
                .to_vec()
        })
        .collect();

    let mut png_images: Vec<PNGImage> = Vec::new();
    for v in data {
        let img = image::load_from_memory_with_format(&v, image::ImageFormat::PNG).unwrap();
        png_images.push(apng::load_dynamic_image(img).unwrap());
    }

    let mut buf = Vec::new();
    {
        let mut buf_writer = BufWriter::new(&mut buf);

        let config = apng::create_config(&png_images, None).unwrap();
        let mut encoder = Encoder::new(&mut buf_writer, config).unwrap();
        let d_num = frame_speed * (100 as f64);
        let d_den = 100;

        let frame = Frame {
            delay_num: Some(d_num as u16),
            delay_den: Some(d_den),
            ..Default::default()
        };

        match encoder.encode_all(png_images, Some(&frame)) {
            Ok(_n) => log("success apng encode!!!"),
            Err(err) => console_log!("{}", err),
        }
    }

    buf
}
