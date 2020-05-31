#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUV;

out vec2 vUV;
out vec3 vNormal;

uniform mat4 uMVP;

void main(){
    gl_Position = uMVP * vec4(aPos, 1.0);
    vUV = aUV;
    vNormal = normalize(uMVP * vec4(aPos.xyz, 0.0)).xyz;
}