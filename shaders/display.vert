precision highp float;

attribute vec3 position;
attribute vec3 norm;
attribute float elevation;

uniform mat4 rotation;
uniform mat4 model;
uniform mat4 light_model;

varying vec3 v_pos_orig;
varying vec3 v_pos;
varying vec3 v_norm;
varying vec4 v_frag_pos_light_space;
varying float v_elev;

void main() {
  vec4 vertex_position = rotation * vec4(position, 1.);

  v_pos_orig = position;
  v_norm = norm;
  v_pos = vertex_position.xyz;
  v_frag_pos_light_space = light_model * vertex_position;
  v_elev = elevation;

  gl_Position = model * vertex_position;
}