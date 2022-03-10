precision highp float;

varying vec2 v_coord;

uniform sampler2D depth_map;

void main() {
  float depth_value = texture2D(depth_map, v_coord).r;
  gl_FragColor = vec4(vec3(depth_value), 1.0);
}