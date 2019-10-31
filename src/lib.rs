mod utils;

use apng;
use apng::Encoder;
use apng::{Frame, PNGImage};
use serde::{Deserialize, Serialize};
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
pub fn apng_encode_all(array: js_sys::Array) {
    console_log!("encoding in apng_encode_all");

    let array: Vec<Vec<u8>> = array
        .values()
        .into_iter()
        .map(|x| {
            x.unwrap_throw()
                .unchecked_ref::<js_sys::Uint8Array>()
                .to_vec()
        })
        .collect();

    console_log!("{:?}", array);

    //let mut vv = vec![];
    //array.values().into_iter().map(|x| {
    //    let mut v = vec![];
    //    x.unwrap_throw()
    //        .unchecked_ref::<js_sys::Uint8Array>()
    //        .copy_to(&mut v[..]);
    //    vv.push(v);
    //});

    //console_log!("{:?}", vv);

    //console_log!("array: {:?}", array);

    //for val in array.values() {
    //    let v = val.unwrap();
    //    console_log!("v: {:?}", v);
    //    //let uint8array: js_sys::Uint8Array = wasm_bindgen::JsCast::unchecked_from_js(v);
    //    //console_log!("uint8array: {:?}", uint8array);
    //    //let uint8array = v.dyn_into::<js_sys::Uint8Array>().unwrap().buffer();
    //    //console_log!("uint8array: {:?}", uint8array);
    //    //let uint8array = v.unchecked_into::<js_sys::Uint8Array>();
    //    //console_log!("uint8array: {:?}", uint8array);
    //}
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
