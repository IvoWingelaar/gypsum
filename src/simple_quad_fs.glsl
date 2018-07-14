precision mediump float;

in vec2 frag_tex;

out vec4 FragColor;

void main() {
    FragColor = vec4(fract(frag_tex.xy * 20.0), 0.5, 0.0);
}
