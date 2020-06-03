#version 460 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUV;
layout (location = 3) in vec4 aTangent;

out vec2 vUV;

void main(){
    gl_Position = vec4(aPos.xyz, 1.0);

    vUV = aUV;
}