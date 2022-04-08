precision highp float;

attribute vec3 position;
attribute vec2 uv;

uniform mat4 rotation;
uniform mat4 light_model;
uniform sampler2D height_map;
uniform float extrude_scale;

vec3 extrude(vec3 point, vec2 uv) {
  return point + point * extrude_scale * texture2D(height_map, uv).r;
}

void main() {
  vec3 pos = extrude(position.xyz, uv);
  vec4 vertex_position = rotation * vec4(pos, 1.);

  gl_Position = light_model * vertex_position;
}