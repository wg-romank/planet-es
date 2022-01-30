import * as fl from "florest";
import * as UIL from "uil";

let canvas = document.getElementById("florest-canvas");
const brect = canvas.getBoundingClientRect();

canvas.setAttribute('width', brect.width);
canvas.setAttribute('height', brect.height);

let r = null;
let arr = window.location.href.split("?");
if (arr.length > 1 && arr[1] != '') {
  let raw = arr[1].split("=")[1];
  let parameters_string = atob(raw)
  r = fl.Render.from("florest-canvas", parameters_string);
} else {
  r = fl.Render.new("florest-canvas");
}

let parameters = JSON.parse(r.parameters());

let lastCall = 0;
let cum = 0;

const fps = 60;

const renderLoop = (timestamp) => {
  const delta = timestamp - lastCall;
  lastCall = timestamp;
  cum += delta;

  if (cum > 1000 / fps) {
    r.frame(lastCall / 1000, JSON.stringify(parameters));

    cum = 0;
  }

  requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);

const hex2rgba = (hex) => {
  const red = parseInt(hex.slice(2, 4), 16) / 255;
  const green = parseInt(hex.slice(4, 6), 16) / 255;
  const blue = parseInt(hex.slice(6, 8), 16) / 255;

  return [red, green, blue, 1.0];
}

const set_parameters = (p) => {
  let url = new URL(window.location)
  url.searchParams.set('p', btoa(JSON.stringify(p)))
  window.history.pushState({}, '', url)
}

let ui = new UIL.Gui({w: 300});
ui.add('title', { name:'Гуга-Муга'});
// ui.add('bool', { name:'Bool' })
ui.add('color', { name:'Color', type:'rgba', value: parameters.color }).onChange(c => {
  parameters.color = hex2rgba(c)
  set_parameters(parameters)
});

ui.add('slide', { name: 'Face resolution', value: parameters.face_resolution, min: 0, max: 128, precision: 0}).onChange(fr => {
  parameters.face_resolution = fr
  set_parameters(parameters)
})

ui.add('slide', { name: 'Noise weight', value: parameters.noise_weight, min: 0, max: 1, step: 0.01}).onChange(nv => {
  parameters.noise_weight = nv
  set_parameters(parameters)
})

ui.add('slide', { name: 'Frequency', value: parameters.frequency, min: 0, max: 10, step: 0.01}).onChange(f => {
  parameters.frequency = f
  set_parameters(parameters)
})

ui.add('slide', {name: 'Octaves', value: parameters.octaves, min: 1, max: 8, step: 1}).onChange(o => {
  parameters.octaves = o
  set_parameters(parameters)
})

ui.add('slide', { name: 'Lacunarity', value: parameters.lacunarity, min: 0, max: 10, step: 0.01}).onChange(l => {
  parameters.lacunarity = l
  set_parameters(parameters)
})

ui.add('slide', { name: 'Gain', value: parameters.frequency, min: 0, max: 3, step: 0.01}).onChange(g => {
  parameters.frequency = g
  set_parameters(parameters)
})
// const obj = {
//   name:'welcome to uil',
//   value: 2,
//   slider: 30,
//   vector: { x:10, y:-30 }
// };
  
// ui.add( obj, 'string', { type:'string' });
// ui.add( obj, 'value', { type:'number', min:0, max:10, precision:2, step:0.01 });
// ui.add( obj, 'slider', { type:'slide' });
// ui.add( obj, 'vector', { type:'number' });