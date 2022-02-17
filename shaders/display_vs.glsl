precision highp float;

in vec3 position;
in vec3 norm;
in float elevation;

uniform mat4 rotation;
uniform mat4 projection;
uniform mat4 light_projection;
uniform mat4 view;
uniform mat4 light_view;

uniform float min_height;
uniform float max_height;
uniform float radius;

out vec3 v_pos;
out vec3 v_norm;
out vec4 v_frag_pos_light_space;
out float v_elev;

void main() {
  vec4 vertex_position = rotation * vec4(position, 1.);
  // float elevation = length(position - normalize(position) * radius);
  // float elevation_scaled = (elevation - min_height) / (max_height - min_height);

  v_norm = norm;
  v_pos = vertex_position.xyz;
  v_frag_pos_light_space = light_projection * light_view * vertex_position;
  // v_elev = elevation_scaled;
  v_elev = elevation;

  gl_Position = projection * view * vertex_position;
}