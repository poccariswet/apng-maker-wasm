import * as wasm from "apng-maker-wasm";


/*
 * struct {
 *  array: Uint8Array,
 *  frame_speed: 0 ~ 1
 * }
 * */

//TODO: frame speed control bar
//      一括で操作もできるし、フレームごとにも設定できる

async function readFile(file) {
  const f = await new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.addEventListener('loadend', () => resolve(reader.result));
    reader.addEventListener('error', reject);
    reader.readAsArrayBuffer(file);
  });

  const array = new Uint8Array(f)
  var img = new Image();
  img.src = uint8ArrayToBase64(array);
  document.getElementById('preview').appendChild(img);

  return array;
}

function uint8ArrayToBase64(buffer) {
  var blob = new Blob([buffer], {type: 'image/png'});
  var url = window.URL.createObjectURL(blob)
  return url
}

// Encode all png to APNG
async function encode(files, frame_speed) {
  let buffers = [];
  for (let i = 0; i<files.length; i++) {
    let buffer = await readFile(files[i])
    buffers.push(buffer)
  }

  console.log('encode proccessing');
  let buffer = wasm.apngEncodeAll(buffers, frame_speed);
  var blob = new Blob([buffer], {type: 'image/png'});
  var url = window.URL.createObjectURL(blob);
  var elem = document.getElementById("apng");
  elem.src = url;
}


document.getElementById('file_input').onchange = function() {
  let files = document.getElementById('file_input').files;
  let frame_speed = document.getElementById('frame_speed').value;

  encode(files, frame_speed)
}


var slider = document.getElementById("frame_speed");
var output = document.getElementById("frame_speed_value");
output.innerHTML = slider.value; // Display the default slider value

// Update the current slider value (each time you drag the slider handle)
slider.oninput = function() {
  output.innerHTML = this.value;
}
