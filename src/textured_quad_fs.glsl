precision mediump float;

in vec2 frag_tex;

uniform sampler2D input_texture;

out vec4 FragColor;

void main() {
    vec4 the_sample = texture(input_texture, frag_tex.xy);

    FragColor = vec4(the_sample.rgb, 0.0);
}
