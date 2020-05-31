#version 460 core

out vec4 FragColor;

in vec3 vPos;
in vec3 vNormal;
in vec2 vUV;

uniform sampler2D uAlbedoTexture;
uniform sampler2D uNormalTexture;
uniform sampler2D uARMTexture;

uniform float uTime;

const vec3 LIGHT_POSITION = vec3(1.0, 1.0, 1.0);

void main(){
    vec4 albedo = texture2D(uAlbedoTexture, vUV).rgba;
    vec3 normal = texture2D(uNormalTexture, vUV).xyz;
    vec3 arm = texture2D(uARMTexture, vUV).xyz;

    vec3 col = albedo.rgb;

    vec3 n = normalize(vNormal);
    vec3 lightDir = normalize(LIGHT_POSITION - vPos);

    float f = min(max(0.0, dot(normal, lightDir)), 1.0);
    col *= f;

    col *= arm.x;

    FragColor = vec4(col, 1.0);
}