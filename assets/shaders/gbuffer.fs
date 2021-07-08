#version 460 core

layout (location = 0) out vec4 gPosition;
layout (location = 1) out vec4 gNormal;
layout (location = 2) out vec4 gAlbedoRoughness;

in vec3 vPos;
in vec3 vNormal;
in vec2 vUV;
in vec3 vTangent;
in vec3 vBinormal;

uniform sampler2D uAlbedoTexture;
uniform sampler2D uNormalTexture;
uniform sampler2D uRoughnessTexture;
uniform sampler2D uMetallicTexture;

void main(){
    vec3 albedo = pow(texture(uAlbedoTexture, vUV).rgb, vec3(2.2));

    mat3 tbn = mat3(normalize(vTangent), normalize(vBinormal), normalize(vNormal));
    vec3 normal = texture(uNormalTexture, vUV).xyz * 2.0 - 1.0;
    normal = normalize(tbn * normal);

    float roughness = texture(uRoughnessTexture, vUV).r;
    float metallic = texture(uMetallicTexture, vUV).r;

    gPosition.rgb = vPos.xyz;
    gNormal.rgb = normal.xyz;
    gAlbedoRoughness.rgb = albedo.rgb;
    gAlbedoRoughness.a = roughness;
}