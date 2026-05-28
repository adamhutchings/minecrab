#version 330 core

in vec2 fragTexCoord;
in vec3 fragVertexNormal;

// Produce a fragment color.
out vec4 fragColor;

uniform sampler2D tex;

void main() {
    fragColor = vec4(0.4, 0.4, 1.0, 1.0);
}
