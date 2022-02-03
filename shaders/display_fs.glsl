in vec3 v_norm;
in vec4 v_frag_pos_light_space;

uniform mat4 normalMatrix;
uniform vec3 lightPosition;
uniform vec4 color;
uniform sampler2D shadow_map;

out vec4 frag_color;

float shadow_calc() {
  vec3 pos = v_frag_pos_light_space.xyz * .5 + .5;
  float depth = texture(shadow_map, pos.xy).r;
  return depth < pos.z ? 0.0 : 1.0;
}

void main() {
  vec3 norm_transformed = (normalMatrix * vec4(v_norm, 0.)).xyz;
  float kd = dot(norm_transformed, -normalize(lightPosition)) + .3;
  
  // float shadow = shadow_calc();
  float shadow = 1.0;

  frag_color = shadow * color * kd;
}