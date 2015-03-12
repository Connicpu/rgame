#version 330

uniform sampler2D texture;

in vec4 v_color;
in vec2 v_tex_coord;

void main() {
    gl_FragColor = v_color * texture2D(texture, v_tex_coord);
}
