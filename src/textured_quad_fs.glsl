precision mediump float;

uniform sampler2D sprites;
uniform sampler2D fore_color;
uniform sampler2D back_color;

in vec2 frag_tex;

out vec4 FragColor;

void main() {
    vec4 the_sample = texture(sprites, frag_tex.xy);
    FragColor = vec4(the_sample.rgb, 1.0);
}
