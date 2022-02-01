in vec3 v_norm;

uniform mat4 normalMatrix;
uniform vec3 lightPosition;
uniform vec4 color;

out vec4 frag_color;

void main() {
  vec3 norm_transformed = (normalMatrix * vec4(v_norm, 0.)).xyz;
  float kd = dot(norm_transformed, -normalize(lightPosition)) + .3;

  frag_color = color * kd;
}