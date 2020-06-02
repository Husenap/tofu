#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUV;
layout (location = 3) in vec4 aTangent;

out vec3 vPos;
out vec3 vNormal;
out vec2 vUV;
out vec3 vTangent;
out vec3 vBinormal;

uniform mat4 uModelMatrix;
uniform mat4 uNormalMatrix;
uniform mat4 uModelViewProjectionMatrix;

void main(){
    vec3 p = aPos;

    gl_Position = uModelViewProjectionMatrix * vec4(p, 1.0);
    vPos = normalize(uModelMatrix * vec4(p, 1.0)).xyz;

    vUV = aUV;

    vNormal = normalize((uNormalMatrix * vec4(aNormal, 0.0)).xyz);
    vTangent = normalize((uNormalMatrix * vec4(aTangent.xyz, 0.0)).xyz);
    vBinormal = normalize(cross(vNormal, vTangent)) * aTangent.w;
}