#version 330

uniform sampler2D texture;

varying vec4 v_color;
varying vec2 v_tex_coord;

void main() {
    gl_FragColor = v_color * texture2D(texture, v_tex_coord);
}
