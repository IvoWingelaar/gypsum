in vec2 vert_pos;
in vec2 vert_tex;

uniform float kValue;

out vec2 frag_tex;

void main(void) {
    gl_Position = vec4(kValue * vert_pos.x, vert_pos.y, 0.0, 1.0);

    frag_tex = vert_tex;
}
