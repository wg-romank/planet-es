in vec3 position;
in vec3 norm;

uniform mat4 rotation;
uniform mat4 projection;
uniform mat4 view;
uniform vec4 color;

out vec3 v_norm;
out vec4 v_color;

void main() {
  vec4 vertex_position = rotation * vec4(position, 1.);
  // vec4 vertex_position = vec4(position, 1.);

  v_color = color;
  v_norm = norm;

  gl_Position = view * vertex_position;
}