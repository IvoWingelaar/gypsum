use program;
use quad::Quad;
use webgl::{WebGLRenderingContext};

pub type Color = (u8, u8, u8, u8);

pub struct Grid {
    _width: usize,
    _height: usize,
    _data: Vec<u8>,
    _fore: Vec<Color>,
    _back: Vec<Color>,
    quad: Quad,
}

impl Grid {
    pub fn new(gl: &WebGLRenderingContext, width: usize, height: usize) -> Self {
        let size = width * height;

        let fs = include_str!("textured_quad_fs.glsl");
        let vs = include_str!("textured_quad_vs.glsl");

        let program = program::create_program(&gl, vs, fs);

        let pos = gl.get_attrib_location(&program, "vert_pos").unwrap();
        let tex = gl.get_attrib_location(&program, "vert_tex").unwrap();

        Grid {
            _width: width,
            _height: height,
            _data: vec![0; size],
            _fore: vec![(0, 0, 0, 0); size],
            _back: vec![(0, 0, 0, 0); size],
            quad: Quad::new(&gl, program, pos, tex),
        }
    }

    pub fn draw(&self, gl: &WebGLRenderingContext) {
        self.quad.update_uniforms(&gl, 1.0);
        self.quad.draw(&gl);
    }
}
