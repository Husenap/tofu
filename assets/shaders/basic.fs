#version 460 core

out vec4 FragColor;

in vec2 vUV;
in vec3 vNormal;

uniform sampler2D texture1;
uniform sampler2D texture2;

uniform float uTime;

void main(){
    vec4 col1 = texture2D(texture1, vUV.xy);
    col1 *= col1.a;

    vec4 col2 = texture2D(texture2, vUV.xy);
    col2 *= col2.a;

    float f = max(0.0, dot(vNormal, vec3(0.0, 1.0, 0.0)));
    f = 0.1 + 0.9*f;

    FragColor = mix(col1, col2, smoothstep(0.1, 0.9, vUV.x)) * f;
}