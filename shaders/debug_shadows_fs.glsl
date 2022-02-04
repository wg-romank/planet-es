in vec2 v_coord;

uniform sampler2D depth_map;

out vec4 frag_color;

void main() {
  float depth_value = texture(depth_map, v_coord).r;
  frag_color = vec4(vec3(depth_value), 1.0);
}