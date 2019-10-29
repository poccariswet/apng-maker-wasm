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
      console.log(e.target.result)
      console.log(new Uint8Array(e.target.result))
      readFile(index+1)
    }
    reader.readAsArrayBuffer(file);
  }
  readFile(0);

  console.log("yaa")
  console.log(typeof file_bufs)
  console.log(file_bufs)

  setTimeout(() => {
      setTimeout(() => {
          console.log('proccessingggggggg');
          let buffer = wasm.apngEncode(file_bufs[0], file_bufs[1])
          console.log(buffer)
          var blob = new Blob([buffer], {type: 'image/png'});
          var url = window.URL.createObjectURL(blob);
          console.log(url)
          window.open(url);

      }, 2000);
  }, 1000);

}
