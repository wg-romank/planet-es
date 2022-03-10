precision highp float;

attribute vec2 position;
attribute vec2 uv;

varying vec2 v_coord;

void main() {
  v_coord = uv;

  gl_Position = vec4(position, 0.0, 1.0);
}