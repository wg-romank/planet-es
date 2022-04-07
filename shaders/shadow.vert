precision highp float;

attribute vec3 position;
attribute vec2 uv;

uniform mat4 rotation;
uniform mat4 light_model;
uniform sampler2D height_map;

void main() {
  vec3 pos = position + mix(0., 0.1, texture2D(height_map, uv).r);
  // vec3 pos = position.xyz;
  vec4 vertex_position = rotation * vec4(pos, 1.);

  gl_Position = light_model * vertex_position;
}