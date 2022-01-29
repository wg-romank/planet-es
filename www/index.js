import * as fl from "florest";
import * as UIL from "uil";

let canvas = document.getElementById("florest-canvas");
const brect = canvas.getBoundingClientRect();

canvas.setAttribute('width', brect.width);
canvas.setAttribute('height', brect.height);

let r = fl.Render.new("florest-canvas");
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

let ui = new UIL.Gui({w: 300});
ui.add('title', { name:'Title'});
// ui.add('bool', { name:'Bool' })
ui.add('color', { name:'Color', type:'rgba', value:[0,1,1,1]}).onChange(c => {
  parameters.color = hex2rgba(c)
});


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