#version 430

uniform mat4 camera;

in vec2 position;
in vec2 tex_coord;
in vec4 color;

out vec4 v_color;
out vec2 v_tex_coord;

void main() {
    gl_Position = vec4(position, 0.0, 1.0) * camera;
    v_tex_coord = tex_coord;
    v_color = color;
}
