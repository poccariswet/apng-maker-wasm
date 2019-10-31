import * as wasm from "apng-maker-wasm";


async function readFile(file) {
  const f = await new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.addEventListener('loadend', () => resolve(reader.result));
    reader.addEventListener('error', reject);
    reader.readAsArrayBuffer(file);
  });
  return new Uint8Array(f);
}

async function encode(files) {
  let buffers = [];
  for (let i = 0; i<files.length; i++) {
    let buffer = await readFile(files[i])
    buffers.push(buffer)
  }

  console.log('encode proccessing');
  let buffer = wasm.apngEncodeAll(buffers);
  var blob = new Blob([buffer], {type: 'image/png'});
  var url = window.URL.createObjectURL(blob);
  var elem = document.getElementById("apng");
  elem.src = url;
}


document.getElementById('file_input').onchange = function() {
  let files = document.getElementById('file_input').files;

  encode(files)
}

