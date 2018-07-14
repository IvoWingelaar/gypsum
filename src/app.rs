use file;
use image;
use program;
use quad;
use uni_app;
use webgl;

pub struct App {
    _width: u32,
    _height: u32,
    app: Option<uni_app::App>,
    gl: webgl::WebGLRenderingContext,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        use std::env;
        env::set_var("WINIT_HIDPI_FACTOR", "1.0");

        let app = uni_app::App::new(uni_app::AppConfig {
            size: (width, height),
            title: "test".into(),
            vsync: false,
            show_cursor: true,
            headless: false,
            resizable: false,
            fullscreen: false,
        });

        let gl = webgl::WebGLRenderingContext::new(app.canvas());
        gl.viewport(0, 0, width, height);

        Self {
            _width: width,
            _height: height,
            app: Some(app),
            gl,
        }
    }

    fn texture_stuff(&self, file: &mut uni_app::fs::File) -> bool {
        if !file.is_ready() {
            return false;
        }

        let tex = self.gl.create_texture();
        self.gl.bind_texture(&tex);

        use webgl::TextureKind::Texture2d;
        use webgl::TextureParameter::*;
        use webgl::{TextureMagFilter, TextureMinFilter};

        self.gl.tex_parameteri(
            Texture2d,
            TextureMagFilter,
            TextureMagFilter::Nearest as i32,
        );
        self.gl.tex_parameteri(
            Texture2d,
            TextureMinFilter,
            TextureMinFilter::Nearest as i32,
        );

        let wrap = webgl::TextureWrap::ClampToEdge as i32;

        self.gl.tex_parameteri(Texture2d, TextureWrapS, wrap);
        self.gl.tex_parameteri(Texture2d, TextureWrapT, wrap);

        let content = file.read_binary().unwrap();
        let im = image::load_from_memory(&content).unwrap().to_rgba();

        self.upload_image(im.width() as u16, im.height() as u16, &im);
        true
    }

    fn upload_image(&self, width: u16, height: u16, data: &[u8]) {
        self.gl.tex_image2d(
            webgl::TextureBindPoint::Texture2d,
            0,
            width,
            height,
            webgl::PixelFormat::Rgba,
            webgl::PixelType::UnsignedByte,
            data,
        );
    }

    pub fn run(mut self) {
        let fs = include_str!("textured_quad_fs.glsl");
        let vs = include_str!("textured_quad_vs.glsl");

        let program = program::create_program(&self.gl, vs, fs);

        let pos = self.gl.get_attrib_location(&program, "vert_pos").unwrap();
        let tex = self.gl.get_attrib_location(&program, "vert_tex").unwrap();

        let quad = quad::Quad::new(&self.gl, program, pos, tex);

        let mut f = file::new("simple-4x4.png");
        let mut texture_loaded = self.texture_stuff(&mut f);

        let mut mousepos = (0.0, 0.0);

        let app = self.app.take().unwrap();
        app.run(move |app: &mut uni_app::App| {
            if !texture_loaded {
                texture_loaded = self.texture_stuff(&mut f);
            }

            self.gl.clear_color(0.5, 0.5, 0.5, 1.0);
            self.gl.clear(webgl::BufferBit::Color);

            for i in app.events.borrow().iter() {
                if let uni_app::AppEvent::MousePos(ref pos) = i {
                    mousepos = *pos;
                }
            }

            quad.update_uniforms(&self.gl, 1.0);
            quad.draw(&self.gl);
        });
    }
}
