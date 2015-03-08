#version 110

uniform mat4 camera;

attribute vec2 position;
attribute vec2 tex_coord;
attribute vec4 color;

varying vec4 v_color;
varying vec2 v_tex_coord;

void main() {
    gl_Position = vec4(position, 0.0, 1.0) * camera;
    v_tex_coord = tex_coord;
    v_color = color;
}
