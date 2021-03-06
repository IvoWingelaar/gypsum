in vec2 vert_pos;
in vec2 vert_tex;

out vec2 frag_tex;

uniform vec2 top_left;
uniform vec2 scale;

void main(void) {
    vec2 tl = top_left * vec2(2.0, -2.0);
    vec2 pos = vec2(-1.0, 1.0) + tl + vert_pos.xy * scale;

    gl_Position = vec4(pos, 0.0, 1.0);

    frag_tex = vert_tex;
}
