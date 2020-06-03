#version 460 core

out vec4 FragColor;

in vec2 vUV;

uniform sampler2D uPositionTexture;
uniform sampler2D uNormalTexture;
uniform sampler2D uAlbedoRoughnessTexture;

void main(){
    vec2 uv = vUV*2.0;

    vec3 position = texture(uPositionTexture, uv).rgb;
    vec3 normal = texture(uNormalTexture, uv).rgb;
    vec3 albedo = pow(texture(uAlbedoRoughnessTexture, uv).rgb, vec3(0.4545));
    float roughness = texture(uAlbedoRoughnessTexture, uv).a;

    vec3 top_row = mix(
        position,
        normal,
        step(0.5, vUV.x)
    );
    vec3 bottom_row = mix(
        albedo,
        vec3(roughness),
        step(0.5, vUV.x)
    );

    vec3 col = mix(
        bottom_row,
        top_row,
        step(0.5, vUV.y)
    );

    vec2 line = abs(vUV-0.5);

    col = mix(
        vec3(1.0, 0.0, 0.0),
        col,
        step(0.005, min(line.x, line.y))
    );

    FragColor = vec4(col, 1.0);
}