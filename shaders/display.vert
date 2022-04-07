precision highp float;

attribute vec3 position;
attribute vec3 norm;
attribute float elevation;
attribute vec2 uv;

uniform mat4 rotation;
uniform mat4 model;
uniform mat4 light_model;
uniform sampler2D height_map;

varying vec3 v_pos_orig;
varying vec3 v_pos;
varying vec3 v_norm;
varying vec4 v_frag_pos_light_space;
varying float v_elev;
varying vec2 v_uv;

void main() {
  vec3 pos = position + mix(0., 0.1, texture2D(height_map, uv).r);
  // vec3 pos = position.xyz;
  vec4 vertex_position = rotation * vec4(pos, 1.);

  v_pos_orig = pos;
  v_norm = norm;
  v_pos = vertex_position.xyz;
  v_frag_pos_light_space = light_model * vertex_position;
  v_elev = elevation;
  v_uv = uv;

  gl_Position = model * vertex_position;
}