export default `#version 100

precision mediump float;

uniform float iTime;
uniform vec3 iResolution;
uniform vec4 iMouse;

void main() {
    gl_FragColor = vec4(1.0 , 0.0, 0.0, 1.0);
}`;