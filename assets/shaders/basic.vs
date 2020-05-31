#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUV;

out vec2 vUV;
out vec3 vNormal;

uniform mat4 uModel;
uniform mat4 uModelViewProjection;

void main(){
    gl_Position = uModelViewProjection * vec4(aPos, 1.0);
    vUV = aUV;
    vNormal = normalize(uModel * vec4(aNormal, 0.0)).xyz;
}