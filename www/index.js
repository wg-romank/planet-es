import * as fl from "florest";
import * as UIL from "uil";

let canvas = document.getElementById("florest-canvas");
const brect = canvas.getBoundingClientRect();

canvas.setAttribute('width', brect.width);
canvas.setAttribute('height', brect.height);

let parameters_string = "{}";

let arr = window.location.href.split("?");
if (arr.length > 1 && arr[1] != '') {
  let raw = arr[1].split("=")[1];
  parameters_string = atob(decodeURIComponent(raw))
}

let r = fl.WebApp.from("florest-canvas", parameters_string);

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

const hex2rgb = (hex) => {
  const red = parseInt(hex.slice(2, 4), 16) / 255;
  const green = parseInt(hex.slice(4, 6), 16) / 255;
  const blue = parseInt(hex.slice(6, 8), 16) / 255;

  return [red, green, blue];
}

const addVectorGroup = (parent, params, name) => {
  let g = parent.add('group', {name: name })

  g.add('slide', {name: 'Center X', value: params.x, min: -10, max: 10, step: 0.01}).onChange(x => {
    params.x = x
  })

  g.add('slide', {name: 'Center Y', value: params.y, min: -10, max: 10, step: 0.01}).onChange(y => {
    params.y = y
  })

  g.add('slide', {name: 'Center Z', value: params.z, min: -10, max: 10, step: 0.01}).onChange(z => {
    params.z = z
  })
}

let ui = new UIL.Gui({w: 300});
ui.add('title', { name:'Гуга-Муга'});

ui.add('slide', { name: 'Detail', value: parameters.face_resolution, min: 1, max: 8, precision: 0}).onChange(fr => {
  parameters.face_resolution = fr
})

ui.add('slide', { name: 'Radius', value: parameters.radius, min: 0, max: 1, step: 0.01}).onChange(r => {
  parameters.radius = r
})

ui.add('list', {name: 'Mode', list: ['Normals', 'Uvs', 'Display'], value: parameters.mode}).onChange(m => {
  parameters.mode = m
})

ui.add('slide', { name: 'FOV', value: parameters.fov, min: 45, max: 180, step: 1}).onChange(fov => {
  parameters.fov = fov
})

ui.add('slide', { name: 'Rotate X', value: parameters.rotate_x_speed, min: 0, max: 1, step: 0.01}).onChange(r => {
  parameters.rotate_x_speed = r
})

ui.add('slide', { name: 'Rotate Y', value: parameters.rotate_y_speed, min: 0, max: 1, step: 0.01}).onChange(r => {
  parameters.rotate_y_speed = r
})

let light = ui.add('group', { name: 'Light'})

let diffuse = light.add('group', { name: 'Diffuse' })

diffuse.add('slide', { name: 'Width', value: parameters.light.diffuse.width, min: 0, max: 10, step: 0.01}).onChange(w => {
  parameters.light.diffuse.width = w
})

diffuse.add('slide', { name: 'Near', value: parameters.light.diffuse.near_clip, min: -10, max: 10, step: 0.01}).onChange(n => {
  parameters.light.diffuse.near_clip = n
})

diffuse.add('slide', { name: 'Far', value: parameters.light.diffuse.far_clip, min: 0, max: 10, step: 0.01}).onChange(f => {
  parameters.light.diffuse.far_clip = f
})

diffuse.add('bool', { name: 'Debug shadows', value: parameters.debug_shadows }).onChange(d => {
  parameters.debug_shadows = d
})

addVectorGroup(diffuse, parameters.light.diffuse.position, 'Position')

light.open();

let meshFilters = ui.add('group', {name: 'Mesh Filters'});

const addFilter = (parent, filterParameters) => {
  let filter = parent.add('group', { name: 'Filter'});

  filter.add('list', {name: 'Type', list: ['Plain', 'Ridge'], value: filterParameters.tup}).onChange(t => {
    filterParameters.tup = t
  })

  filter.add('slide', {name: 'Strength', value: filterParameters.strength, min: -1, max: 1, step: 0.01}).onChange(s => {
    filterParameters.strength = s
  })

  filter.add('slide', {name: 'Roughness', value: filterParameters.roughness, min: 0, max: 10, step: 0.01}).onChange(r => {
    filterParameters.roughness = r
  })

  filter.add('slide', {name: 'Threshold', value: filterParameters.min_value, min: 0, max: 1, step: 0.01}).onChange(t => {
    filterParameters.min_value = t
  })

  addVectorGroup(filter, filterParameters.center, 'Center')

  filter.add('bool', { name: 'Enabled', value: filterParameters.enabled }).onChange(en => {
    filterParameters.enabled = en
  })

  filter.add('button', { name: 'Remove'}).onChange(_ => {
    parameters.mesh_parameters.filters = parameters.mesh_parameters.filters.filter(f => f != filterParameters)
    meshFilters.remove(filter)
  })

  parent.open()
}

meshFilters.add('bool', { name: 'Mask', value: parameters.mesh_parameters.use_first_layer_as_mask}).onChange(m => {
  parameters.mesh_parameters.use_first_layer_as_mask = m
})

meshFilters.add('slide', { name: 'Frequency', value: parameters.mesh_parameters.frequency, min: 0., max: 1., step: 0.01}).onChange(f => {
  parameters.mesh_parameters.frequency = f
})

parameters.mesh_parameters.filters.forEach(f => addFilter(meshFilters, f))

ui.add('button', {name: 'Add Filter'}).onChange(() => {
  let params = JSON.parse(fl.MeshFilterParameters.generate())
  parameters.mesh_parameters.filters.push(params)
  addFilter(meshFilters, params)
})

let textureParameters = ui.add('group', {name: 'Texture'});

const addHeight = (parent, heightParameters) => {
  let th = parent.add('group', { name: 'Height level'} )
  th.add('slide', {name: 'Height', value: heightParameters.max_height, min: 0, max: 1, step: 0.01}).onChange(h => {
    heightParameters.max_height = h
  })

  th.add('color', { name:'Color', type:'rgba', value: heightParameters.color }).onChange(c => {
    heightParameters.color = hex2rgb(c)
  }) 

  th.add('button', { name: 'Remove'}).onChange(_ => {
    parameters.texture_parameters.heights = parameters.texture_parameters.heights.filter(f => f != heightParameters)
    textureParameters.remove(filter)
  })

  parent.open()
}

parameters.texture_parameters.heights.forEach(h => addHeight(textureParameters, h))

ui.add('button', {name: 'Add Height'}).onChange(() => {
  let params = JSON.parse(fl.TextureHeightParameters.generate())
  parameters.texture_parameters.heights.push(params)
  addHeight(textureParameters, params)
})

const set_parameters = (p) => {
  let url = new URL(window.location)
  url.searchParams.set('p', btoa(JSON.stringify(p)))
  window.history.pushState({}, '', url)
}

ui.add('button', {name: 'Save & Share'}).onChange(() => set_parameters(parameters))

ui.add('button', {name: 'Export Model'}).onChange(() => {
  UIL.Files.save({name: 'planet.obj', data: r.export_to_obj(), type: 'text'})
})