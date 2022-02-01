in vec3 v_norm;
in vec4 v_color;

uniform mat4 normalMatrix;
uniform vec3 lightPosition;

out vec4 frag_color;

void main() {
  vec3 norm_transformed = (normalMatrix * vec4(v_norm, 0.)).xyz;
  float kd = dot((norm_transformed, -normalize(lightPosition)) + .3;

  frag_color = v_color * kd;
}