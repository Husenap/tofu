#version 460 core

out vec4 FragColor;

in vec3 vPos;
in vec3 vNormal;
in vec2 vUV;

uniform sampler2D uAlbedoTexture;
uniform sampler2D uNormalTexture;
uniform sampler2D uRoughnessTexture;
uniform sampler2D uMetallicTexture;

uniform float uTime;

void main(){
    float mask = textureLod(uAlbedoTexture, vUV, 0.0).a;
    if(mask < 0.5){
        discard;
    }

    vec3 albedo = pow(texture(uAlbedoTexture, vUV).rgb, vec3(2.2));

    vec3 normal = texture(uNormalTexture, vUV).xyz;
    float roughness = texture(uRoughnessTexture, vUV).r;
    float metallic = texture(uMetallicTexture, vUV).r;

    vec3 n = normalize(vNormal);

    vec3 col = vec3(0.0);

    vec3 sunDirection = normalize(vec3(0.6, 0.35, 0.5));
    float sunDiffuse = clamp(dot(n, sunDirection), 0.0, 1.0);

    float skyDiffuse = sqrt(clamp(0.5+0.5*dot(n, vec3(0.0, 1.0, 0.0)), 0.0, 1.0));

    float bounceDiffuse = clamp(dot(n, vec3(0.0, -1.0, 0.0)), 0.0, 1.0);

    col += sunDiffuse * vec3(8.10, 6.00, 4.20) * 0.5;
    col += skyDiffuse * vec3(0.50, 0.70, 1.00);
    col += bounceDiffuse * vec3(0.20, 0.70, 0.10) * 0.25;

    col *= albedo;

    FragColor = vec4(pow(col, vec3(0.4545)), 1.0);
}