export default `#version 100

precision mediump float;

uniform float iTime;
uniform vec3 iResolution;
uniform vec4 iMouse;

void main() {
  vec2 uv =  gl_FragCoord.xy/iResolution.xy;

  vec3 col = 0.5 + 0.5 * cos(iTime + uv.xyx + vec3(0, 2, 4));

  gl_FragColor = vec4(col, 1.0);
}`;