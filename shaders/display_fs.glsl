in vec3 v_pos;
in vec3 v_norm;
in vec4 v_frag_pos_light_space;

uniform mat4 normalMatrix;
uniform vec3 lightPosition;
uniform vec4 color;
uniform sampler2D shadow_map;

out vec4 frag_color;

float shadow_calc() {
  // [-1, 1] => [0, 1]
  vec3 pos = v_frag_pos_light_space.xyz * .5 + .5;

  float depth = texture(shadow_map, pos.xy).r;
  float bias = 0.05;

  return (depth + bias) < pos.z ? 0.0 : 1.0;
}

void main() {
  //ambient
  vec3 ambient = 0.3 * color.xyz;

  // diffuse
  vec3 light_dir = normalize(lightPosition - v_pos);
  vec3 norm_transformed = (normalMatrix * vec4(v_norm, 0.)).xyz;
  float dot_light_normal = dot(light_dir, norm_transformed);
  float diff = max(dot_light_normal, 0.0);
  vec3 diffuse = diff * color.xyz;

  // vec3 norm_transformed = (normalMatrix * vec4(v_norm, 0.)).xyz;
  // float kd = dot(norm_transformed, -normalize(lightPosition));
  // vec3 diffuse = kd * color.xyz;
  
  float shadow = shadow_calc();
  // float shadow = 1.0;

  vec3 lighting = ((diffuse * shadow) + ambient) * color.xyz;

  frag_color = vec4(lighting, 1.);
  // frag_color = vec4(v_norm, 1.);
}