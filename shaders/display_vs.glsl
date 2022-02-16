in vec3 position;
in vec3 norm;
in vec3 c;

uniform mat4 rotation;
uniform mat4 projection;
uniform mat4 light_projection;
uniform mat4 view;
uniform mat4 light_view;

out vec3 v_c;
out vec3 v_pos;
out vec3 v_norm;
out vec4 v_frag_pos_light_space;

void main() {
  vec4 vertex_position = rotation * vec4(position, 1.);

  v_norm = norm;
  v_pos = vertex_position.xyz;
  v_frag_pos_light_space = light_projection * light_view * vertex_position;
  v_c = c;

  gl_Position = projection * view * vertex_position;
}