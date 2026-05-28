#version 330 core

in vec2 fragTexCoord;
in vec3 fragVertexNormal;

// Produce a fragment color.
out vec4 fragColor;

uniform sampler2D tex;

const vec4 topColor = vec4(0.5, 0.8, 1.0, 1.0);
const vec4 bottomColor = vec4(0.9, 0.9, 0.9, 1.0);

void main() {
    float up = dot(fragVertexNormal, vec3(0.0, -1.0, 0.0));
    // Top and bottom faces
    if (up > 0.9) {
        fragColor = topColor;
    } else if (up < -0.9) {
        fragColor = bottomColor;
    } else {
        // Otherwise, we use the texture coords to determine where we are.
        float amountUp = fragTexCoord[1];
        fragColor = amountUp * topColor + (1 - amountUp) * bottomColor;
    }
}
