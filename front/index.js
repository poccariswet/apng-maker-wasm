import * as wasm from "apng-maker-wasm";

document.getElementById('file_input').onchange = function() {
  let files = document.getElementById('file_input').files;
  console.log(files);

  var fileReader = new FileReader();
  fileReader.onload = function () {
    console.log( fileReader.result ) ; // ArrayBuffer
    wasm.apngEncode(new Uint8Array(fileReader.result));
  }

  // 読み込みを実行
  fileReader.readAsArrayBuffer(files[0]);
}
