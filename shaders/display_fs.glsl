in vec3 v_pos;
in vec3 v_norm;
in vec4 v_frag_pos_light_space;

uniform mat4 normalMatrix;
uniform vec3 lightPosition;
uniform vec4 color;
uniform sampler2D shadow_map;

out vec4 frag_color;

float shadow_calc(float dot_ligth_normal) {
  // [-1, 1] => [0, 1]
  vec3 pos = clamp((v_frag_pos_light_space.xyz + 1.) / 2., 0.0, 1.0);

  float bias = max(0.05 * (1.0 - dot_ligth_normal), 0.005);

  // PCF
  float shadow = 0.0;
  // todo: pass texture size
  vec2 texel_size = vec2(1.0, 1.0) / vec2(400, 400);
  for (int x = -1; x <= 1; ++x) {
    for (int y = -1; y <= 1; ++y) {
      float depth = texture(shadow_map, pos.xy + vec2(x, y) * texel_size).r;
      shadow += (depth + bias) < pos.z ? 0.0 : 1.0;
    }
  }

  // float depth = texture(shadow_map, pos.xy).r;
  // float shadow = (depth + bias) < pos.z ? 0.0 : 1.0;

  return shadow / 9.0;
}

void main() {
  vec3 norm_transformed = (normalMatrix * vec4(v_norm, 0.)).xyz;

  //ambient
  vec3 ambient = 0.3 * color.xyz;

  // diffuse
  vec3 light_dir = normalize(lightPosition - v_pos);
  float dot_light_normal = dot(light_dir, norm_transformed);
  float diff = max(dot_light_normal, 0.0);
  vec3 diffuse = diff * color.xyz;

  // vec3 norm_transformed = (normalMatrix * vec4(v_norm, 0.)).xyz;
  // float kd = dot(norm_transformed, -normalize(lightPosition));
  // vec3 diffuse = kd * color.xyz;
  
  float shadow = shadow_calc(dot_light_normal);
  // float shadow = 1.0;

  vec3 lighting = ((diffuse * shadow) + ambient) * color.xyz;

  frag_color = vec4(lighting, 1.);
  // frag_color = vec4(v_norm, 1.);
}