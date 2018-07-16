precision mediump float;

uniform sampler2D sprites;
uniform sampler2D data;
uniform sampler2D fore_color;
uniform sampler2D back_color;

uniform vec2 row_col_count;
uniform float cells_per_line;

in vec2 frag_tex;

out vec4 FragColor;

void main() {
    float val = texture(data, frag_tex.xy).r * 255.0;
    float row_frac = val / cells_per_line;
    vec2 tile = vec2(fract(row_frac), floor(row_frac) / cells_per_line);

    vec2 grid_coords = fract(frag_tex.xy * row_col_count);

    vec4 c = texture(sprites, tile + grid_coords / cells_per_line);

    vec4 f_color = texture(fore_color, frag_tex.xy);
    vec4 b_color = texture(back_color, frag_tex.xy);

    // Blend colors together.
    FragColor = c.a * vec4(f_color.rgb, 1.0) * vec4(c.rgb, 1.0) + (1.0 - c.a) * vec4(b_color.rgb, 1.0);
}
