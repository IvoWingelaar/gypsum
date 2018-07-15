use program;
use webgl::{AttributeSize, Primitives, WebGLBuffer, WebGLRenderingContext, WebGLVertexArray};

pub struct Quad {
    vao: WebGLVertexArray,
    _pos: (WebGLBuffer, u32),
    _tex: (WebGLBuffer, u32),
}

fn square(left: f32, up: f32, right: f32, down: f32) -> [f32; 8] {
    [left, down, left, up, right, up, right, down]
}

impl Quad {
    pub fn new(gl: &WebGLRenderingContext, pos: u32, tex: u32) -> Self {
        let pos_coords = square(0.0, 1.0, 1.0, 0.0);
        let tex_coords = vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0];

        let vao = gl.create_vertex_array();

        gl.bind_vertex_array(&vao);

        let pos_buf = gl.create_buffer();
        let tex_buf = gl.create_buffer();

        program::set_buffer_data(&gl, &pos_buf, &pos_coords, pos, AttributeSize::Two);
        program::set_buffer_data(&gl, &tex_buf, &tex_coords, tex, AttributeSize::Two);

        Quad {
            vao,
            _pos: (pos_buf, pos),
            _tex: (tex_buf, tex),
        }
    }

    pub fn draw(&self, gl: &WebGLRenderingContext) {
        gl.bind_vertex_array(&self.vao);
        gl.draw_arrays(Primitives::TriangleFan, 4);
    }
}
