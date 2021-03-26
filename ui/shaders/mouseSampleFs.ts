export default `#version 100

precision mediump float;

uniform float iTime;
uniform vec3 iResolution;
uniform vec4 iMouse;

void main()
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = gl_FragCoord.xy/iResolution.xy;
    vec2 ms = iMouse.xy / iResolution.xy;

    // Time varying pixel color
    // vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

    vec2 center = vec2(0.5, 0.5);

    vec3 col = vec3(0.0, 0.0, 0.5);

    col.r = smoothstep( 0.2, 0.21, length(uv - ms) );

    // Output to screen
    gl_FragColor = vec4(col,1.0);
}
`;