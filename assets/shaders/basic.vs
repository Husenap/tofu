#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUV;

out vec3 vPos;
out vec3 vNormal;
out vec2 vUV;

uniform mat4 uModel;
uniform mat4 uInverseModel;
uniform mat4 uModelViewProjection;

void main(){
    gl_Position = uModelViewProjection * vec4(aPos, 1.0);
    vPos = normalize(uModel * vec4(aPos, 1.0)).xyz;
    vNormal = (uInverseModel * vec4(aNormal, 0.0)).xyz;
    vUV = aUV;
}