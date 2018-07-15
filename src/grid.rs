use program;
use quad::Quad;
use texture;
use webgl::{WebGLProgram, WebGLRenderingContext, WebGLTexture, WebGLUniformLocation};

pub type Color = (u8, u8, u8, u8);

pub struct Grid {
    width: usize,
    height: usize,
    _data: Vec<u8>,
    fore: Vec<Color>,
    back: Vec<Color>,

    sprite_loc: Option<WebGLUniformLocation>,

    fore_gl: Option<(WebGLTexture, WebGLUniformLocation)>,
    back_gl: Option<(WebGLTexture, WebGLUniformLocation)>,

    top_left: WebGLUniformLocation,
    scale: WebGLUniformLocation,

    program: WebGLProgram,
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

        let fore = vec![(255, 0, 0, 0); size];
        let back = vec![(0, 255, 0, 0); size];

        use program::AsDataSlice;

        let fore_gl = if let Some(loc) = gl.get_uniform_location(&program, "fore_color") {
            let x = texture::new_and_setup(&gl);
            gl.active_texture(1);
            gl.bind_texture(&x);
            texture::upload_rgba8(&gl, width as u16, height as u16, fore.as_data_slice());

            Some((x, loc))
        } else {
            None
        };

        let back_gl = if let Some(loc) = gl.get_uniform_location(&program, "back_color") {
            let x = texture::new_and_setup(&gl);
            gl.active_texture(2);
            gl.bind_texture(&x);
            texture::upload_rgba8(&gl, width as u16, height as u16, back.as_data_slice());

            Some((x, loc))
        } else {
            None
        };

        let top_left = gl.get_uniform_location(&program, "top_left").unwrap();
        let scale = gl.get_uniform_location(&program, "scale").unwrap();

        Grid {
            width,
            height,
            _data: vec![0; size],
            fore,
            back,
            sprite_loc: gl.get_uniform_location(&program, "sprites"),
            fore_gl,
            back_gl,
            top_left,
            scale,
            program,
            quad: Quad::new(&gl, pos, tex),
        }
    }

    pub fn upload_textures(&self, gl: &WebGLRenderingContext) {
        use program::AsDataSlice;

        if let Some((ref x, _)) = self.fore_gl {
            gl.active_texture(1);
            gl.bind_texture(&x);
            texture::upload_sub_rgba8(
                &gl,
                self.width as u16,
                self.height as u16,
                self.fore.as_data_slice(),
            );
        }

        if let Some((ref x, _)) = self.back_gl {
            gl.active_texture(2);
            gl.bind_texture(&x);
            texture::upload_sub_rgba8(
                &gl,
                self.width as u16,
                self.height as u16,
                self.back.as_data_slice(),
            );
        }
    }

    fn set_uniforms(&self, gl: &WebGLRenderingContext) {
        if let Some(ref x) = self.sprite_loc {
            gl.active_texture(0);
            gl.uniform_1i(x, 0);
        }

        if let Some((ref x, ref y)) = self.fore_gl {
            gl.active_texture(1);
            gl.bind_texture(&x);
            gl.uniform_1i(y, 1);
        }

        if let Some((ref x, ref y)) = self.back_gl {
            gl.active_texture(2);
            gl.bind_texture(&x);
            gl.uniform_1i(y, 2);
        }

        gl.uniform_2f(&self.top_left, (0.0, 0.0));
        gl.uniform_2f(&self.scale, (2.0, 2.0));
    }

    pub fn draw(&self, gl: &WebGLRenderingContext) {
        self.upload_textures(&gl);

        gl.use_program(&self.program);

        self.set_uniforms(&gl);
        self.quad.draw(&gl);
    }
}
