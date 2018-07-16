use program;
use quad::Quad;
use texture;
use webgl::{WebGLProgram, WebGLRenderingContext, WebGLTexture, WebGLUniformLocation};

pub type Color = (u8, u8, u8, u8);

pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<u32>,
    fore: Vec<Color>,
    back: Vec<Color>,

    sprite_loc: Option<WebGLUniformLocation>,

    data_gl: Option<(WebGLTexture, WebGLUniformLocation)>,
    fore_gl: Option<(WebGLTexture, WebGLUniformLocation)>,
    back_gl: Option<(WebGLTexture, WebGLUniformLocation)>,

    top_left: WebGLUniformLocation,
    scale: WebGLUniformLocation,

    row_col_count: WebGLUniformLocation,
    cells_per_line: WebGLUniformLocation,

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

        let data = vec![4; size];
        let fore = vec![(255, 255, 255, 0); size];
        let back = vec![(0, 0, 0, 0); size];

        use program::AsDataSlice;

        let data_gl = if let Some(loc) = gl.get_uniform_location(&program, "data") {
            let x = texture::new_and_setup(&gl);
            gl.active_texture(1);
            gl.bind_texture(&x);
            texture::upload_rgba8(&gl, width as u16, height as u16, data.as_data_slice());

            Some((x, loc))
        } else {
            None
        };

        let fore_gl = if let Some(loc) = gl.get_uniform_location(&program, "fore_color") {
            let x = texture::new_and_setup(&gl);
            gl.active_texture(2);
            gl.bind_texture(&x);
            texture::upload_rgba8(&gl, width as u16, height as u16, fore.as_data_slice());

            Some((x, loc))
        } else {
            None
        };

        let back_gl = if let Some(loc) = gl.get_uniform_location(&program, "back_color") {
            let x = texture::new_and_setup(&gl);
            gl.active_texture(3);
            gl.bind_texture(&x);
            texture::upload_rgba8(&gl, width as u16, height as u16, back.as_data_slice());

            Some((x, loc))
        } else {
            None
        };

        let top_left = gl.get_uniform_location(&program, "top_left").unwrap();
        let scale = gl.get_uniform_location(&program, "scale").unwrap();
        let row_col_count = gl.get_uniform_location(&program, "row_col_count").unwrap();
        let cells_per_line = gl.get_uniform_location(&program, "cells_per_line").unwrap();

        Grid {
            width,
            height,
            data,
            fore,
            back,
            sprite_loc: gl.get_uniform_location(&program, "sprites"),
            data_gl,
            fore_gl,
            back_gl,
            top_left,
            scale,
            row_col_count,
            cells_per_line,
            program,
            quad: Quad::new(&gl, pos, tex),
        }
    }

    pub fn upload_textures(&self, gl: &WebGLRenderingContext) {
        use program::AsDataSlice;

        if let Some((ref x, _)) = self.data_gl {
            gl.active_texture(1);
            gl.bind_texture(&x);
            texture::upload_sub_rgba8(
                &gl,
                self.width as u16,
                self.height as u16,
                self.data.as_data_slice(),
            );
        }

        if let Some((ref x, _)) = self.fore_gl {
            gl.active_texture(2);
            gl.bind_texture(&x);
            texture::upload_sub_rgba8(
                &gl,
                self.width as u16,
                self.height as u16,
                self.fore.as_data_slice(),
            );
        }

        if let Some((ref x, _)) = self.back_gl {
            gl.active_texture(3);
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

        if let Some((ref x, ref y)) = self.data_gl {
            gl.active_texture(1);
            gl.bind_texture(&x);
            gl.uniform_1i(y, 1);
        }

        if let Some((ref x, ref y)) = self.fore_gl {
            gl.active_texture(2);
            gl.bind_texture(&x);
            gl.uniform_1i(y, 2);
        }

        if let Some((ref x, ref y)) = self.back_gl {
            gl.active_texture(3);
            gl.bind_texture(&x);
            gl.uniform_1i(y, 3);
        }

        // TODO: magic numbers
        gl.uniform_2f(&self.top_left, (0.0, 0.0));
        gl.uniform_2f(&self.scale, (2.0, 2.0));
        gl.uniform_2f(&self.row_col_count, (self.width as f32, self.height as f32));
        gl.uniform_1f(&self.cells_per_line, 4.0);
    }

    pub fn update_mouse(&mut self, mouse: (f64, f64)) {
        for i in &mut self.fore {
            *i = (255, 255, 255, 0);
        }

        if mouse.0 >= 0.0 && mouse.0 <= 1.0 && mouse.1 >= 0.0 && mouse.1 <= 1.0 {
            let w = f64::floor(mouse.0 * self.width as f64) as usize;
            let h = f64::floor(mouse.1 * self.height as f64) as usize;

            self.fore[w + h * self.width] = (255, 0, 0, 0);
        }
    }

    pub fn draw(&self, gl: &WebGLRenderingContext) {
        self.upload_textures(&gl);

        gl.use_program(&self.program);

        self.set_uniforms(&gl);
        self.quad.draw(&gl);
    }
}
