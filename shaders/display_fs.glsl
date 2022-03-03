precision highp float;

in vec3 v_pos_orig;
in vec3 v_pos;
in vec3 v_norm;
in vec4 v_frag_pos_light_space;
in float v_elev;

uniform vec3 blend;
uniform mat4 normalMatrix;
uniform vec3 lightPosition;
uniform sampler2D shadow_map;
uniform sampler2D height_map;
uniform mat4 model;
uniform float mode;
uniform float sharpness;
uniform float scale;
uniform sampler2D waves_1;
uniform sampler2D waves_2;

uniform float diffuse_intensity;
uniform float ambient;

out vec4 frag_color;

vec3 triplanar(sampler2D t, vec3 norm, vec3 pos) {
  vec3 normalX = texture(t, scale * pos.zy).xyz;
  vec3 normalY = texture(t, scale * pos.xz).xyz;
  vec3 normalZ = texture(t, scale * pos.xy).xyz;

  // vec3 normalSign = sign(norm);

  vec3 blend = pow(abs(norm), vec3(sharpness));
  blend /= dot(blend, vec3(1.));

  // return normalize(
  //   normalX * blendWeight.x + normalY * blendWeight.y + normalZ * blendWeight.z
  // );

  normalX = vec3(normalX.xy + norm.zy, abs(normalX.z) * norm.x);
  normalY = vec3(normalY.xy + norm.xz, abs(normalY.z) * norm.y);
  normalZ = vec3(normalZ.xy + norm.xy, abs(normalZ.z) * norm.z);

  return normalize(
    normalX.zyx * blend.x +
    normalY.xzy * blend.y +
    normalZ.xyz * blend.z
  );
}

float shadow_calc(float dot_ligth_normal) {
  // [-1, 1] => [0, 1]
  vec3 pos = clamp((v_frag_pos_light_space.xyz + 1.) / 2., 0.0, 1.0);

  float bias = max(0.05 * (1.0 - dot_ligth_normal), 0.005);

  // PCF
  float shadow = 0.0;
  // todo: pass texture size
  vec2 texel_size = vec2(1.0, 1.0) / vec2(800, 800);
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

vec4 color_calc() {
  vec4 color = texture(height_map, vec2(v_elev, 0.));
  return color;
}

void main() {
  vec4 color = color_calc();
  vec3 trip_normal = triplanar(waves_1, v_norm, v_pos_orig);
  // vec3 trip_normal = v_norm;
  vec3 norm_transformed = (normalMatrix * vec4(trip_normal, 0.)).xyz;

  //ambient
  vec3 ambient = ambient * color.xyz;

  // diffuse
  vec3 light_dir = normalize(lightPosition - v_pos);
  float dot_light_normal = dot(light_dir, norm_transformed);
  float diff = max(dot_light_normal, 0.0);
  vec3 diffuse = diffuse_intensity * diff * color.xyz;

  // vec3 norm_transformed = (normalMatrix * vec4(v_norm, 0.)).xyz;
  // float kd = dot(norm_transformed, -normalize(lightPosition));
  // vec3 diffuse = kd * color.xyz;
  
  float shadow = shadow_calc(dot_light_normal);
  // float shadow = 1.0;

  vec3 lighting = ((diffuse * shadow) + ambient) * color.xyz;

  if (mode == 0.) {
    frag_color = vec4(v_norm, 1.);
  }

  if (mode == 1.) {
    frag_color = vec4(trip_normal, 1.);
  }

  if (mode == 2.) {
    frag_color = vec4(lighting, 1.);
  }
}