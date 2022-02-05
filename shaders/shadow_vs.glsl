in vec3 position;

uniform mat4 rotation;
uniform mat4 projection;
// uniform mat4 view;
uniform mat4 light_view;

void main() {
  vec4 vertex_position = rotation * vec4(position, 1.);

  gl_Position = projection * light_view * vertex_position;
}