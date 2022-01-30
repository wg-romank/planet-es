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

let ui = new UIL.Gui({w: 300});
ui.add('title', { name:'Гуга-Муга'});

ui.add('color', { name:'Color', type:'rgba', value: parameters.color }).onChange(c => {
  parameters.color = hex2rgba(c)
});

ui.add('slide', { name: 'Face resolution', value: parameters.face_resolution, min: 0, max: 128, precision: 0}).onChange(fr => {
  parameters.face_resolution = fr
})

ui.add('slide', { name: 'Radius', value: parameters.radius, min: 0, max: 1, step: 0.01}).onChange(r => {
  parameters.radius = r
})

const addFilter = (parent, filterParameters) => {
  let filter = parent.add('group', { name: 'Filter'})

  filter.add('slide', {name: 'Strength', value: filterParameters.strength, min: 0, max: 1, step: 0.01}).onChange(s => {
    filterParameters.strength = s
  })

  filter.add('slide', {name: 'Roughness', value: filterParameters.roughness, min: 0, max: 10, step: 0.01}).onChange(r => {
    filterParameters.roughness = r
  })

  filter.add('slide', {name: 'Threshold', value: filterParameters.min_value, min: 0, max: 1, step: 0.01}).onChange(t => {
    filterParameters.min_value = t
  })

  let filter_center = filter.add('group', {name: 'Center'})

  filter_center.add('slide', {name: 'Center X', value: filterParameters.center.x, min: -10, max: 10, step: 0.01}).onChange(x => {
    filterParameters.center.x = x
  })

  filter_center.add('slide', {name: 'Center Y', value: filterParameters.center.y, min: -10, max: 10, step: 0.01}).onChange(y => {
    filterParameters.center.y = y
  })

  filter_center.add('slide', {name: 'Center Z', value: filterParameters.center.z, min: -10, max: 10, step: 0.01}).onChange(z => {
    filterParameters.center.z = z
  })

  filter.add('bool', { name: 'Enabled', value: filterParameters.enabled }).onChange(en => {
    filterParameters.enabled = en
  })
}

let meshFilters = ui.add('group', {name: 'Mesh Filters'})

parameters.filters.forEach(f => addFilter(meshFilters, f))

ui.add('button', {name: 'Add Filter'}).onChange(() => {
  let params = JSON.parse(fl.MeshFilterParameters.generate())
  parameters.filters.push(params)
  addFilter(meshFilters, params)
})

const set_parameters = (p) => {
  let url = new URL(window.location)
  url.searchParams.set('p', btoa(JSON.stringify(p)))
  window.history.pushState({}, '', url)
}

ui.add('button', {name: 'Save & Share'}).onChange(() => set_parameters(parameters))
