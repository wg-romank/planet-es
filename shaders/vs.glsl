// those are our vertex attributes
in vec3 position;
in vec3 norm;

uniform mat4 rotation;
uniform mat4 normalMatrix;
uniform vec4 color;
uniform vec3 lightPosition;

out vec3 vLighting;
out vec4 vColor;

void main() {
  vec4 pos = rotation * vec4(position, 1.5);
  gl_Position = pos;

  vec3 ambientLight = vec3(0.3, 0.3, 0.3);
  vec3 directionalLightColor = vec3(1, 1, 1);
  vec3 directionalVector = normalize(lightPosition);

  vec4 transformedNormal = normalMatrix * vec4(norm, 0.0);
  float directional = max(dot(transformedNormal.xyz, directionalVector), 0.0);

  vLighting = ambientLight + (directionalLightColor * directional);
  vColor = color;
}