in vec3 position;
in vec3 norm;

uniform mat4 rotation;
uniform mat4 projection;
uniform mat4 view;

out vec3 v_norm;

void main() {
  vec4 vertex_position = rotation * vec4(position, 1.);

  v_norm = norm;

  gl_Position = projection * view * vertex_position;
}