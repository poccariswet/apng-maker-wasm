import * as wasm from "apng-maker-wasm";

document.getElementById('file_input').onchange = function() {
  let files = document.getElementById('file_input').files;
  console.log(files);

  wasm.apng_encode_all();

  wasm.using_macro();
}
