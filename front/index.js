import * as wasm from "apng-maker-wasm";

document.getElementById('file_input').onchange = function() {
  let files = document.getElementById('file_input').files;
  console.log(files);

  let file_bufs = [];
  var reader = new FileReader();
  function readFile(index) {
    if (index >= files.length) return;
    var file = files[index];
    reader.onload = function(e) {
      file_bufs.push(new Uint8Array(e.target.result));
      readFile(index+1)
    }
    reader.readAsArrayBuffer(file);
  }
  readFile(0);


  setTimeout(() => {
    console.log('encode proccessing');
    let buffer = wasm.apngEncode(file_bufs[0], file_bufs[1], file_bufs[2])
    //let buffer = wasm.apngEncodeAll(file_bufs);
    var blob = new Blob([buffer], {type: 'image/png'});
    var url = window.URL.createObjectURL(blob);
    var elem = document.getElementById("apng");
    elem.src = url;

    //window.open(url);

  }, 100);

}
