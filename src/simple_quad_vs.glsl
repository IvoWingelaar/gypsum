in vec2 vert_pos;
in vec2 vert_tex;

out vec2 frag_tex;

void main(void) {
    gl_Position = vec4(vert_pos.xy, 0.0, 1.0);

    frag_tex = vert_tex;
}
