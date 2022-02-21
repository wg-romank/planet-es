in vec3 position;

uniform mat4 rotation;
uniform mat4 light_model;

void main() {
  vec4 vertex_position = rotation * vec4(position, 1.);

  gl_Position = light_model * vertex_position;
}