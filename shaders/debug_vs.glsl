in vec2 position;
in vec2 uv;

out vec2 v_coord;

void main() {
  v_coord = uv;

  gl_Position = vec4(position, 0.0, 1.0);
}