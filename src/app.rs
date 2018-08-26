use file;
use grid::Grid;
use image;
use texture;
use uni_app;
use uni_gl;

pub struct App {
    width: u32,
    height: u32,
    app: Option<uni_app::App>,
    gl: uni_gl::WebGLRenderingContext,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        use std::env;
        env::set_var("WINIT_HIDPI_FACTOR", "1.0");

        let app = uni_app::App::new(uni_app::AppConfig {
            size: (width, height),
            title: "Title".into(),
            vsync: false,
            show_cursor: true,
            headless: false,
            resizable: false,
            fullscreen: false,
        });

        let gl = uni_gl::WebGLRenderingContext::new(app.canvas());
        gl.viewport(0, 0, width, height);

        gl.enable(uni_gl::Flag::Blend as i32);
        gl.blend_equation(uni_gl::BlendEquation::FuncAdd);
        gl.blend_func(
            uni_gl::BlendMode::SrcAlpha,
            uni_gl::BlendMode::OneMinusSrcAlpha,
        );

        Self {
            width,
            height,
            app: Some(app),
            gl,
        }
    }

    fn texture_stuff(&self, file: &mut uni_app::fs::File) -> bool {
        if !file.is_ready() {
            return false;
        }

        let texture = texture::new_and_setup(&self.gl);

        let content = file.read_binary().unwrap();
        let im = image::load_from_memory(&content).unwrap().to_rgba();

        self.gl.active_texture(0);
        self.gl.bind_texture(&texture);

        texture::upload_rgba8(&self.gl, im.width() as u16, im.height() as u16, &im);
        true
    }

    pub fn run(mut self) {
        let mut grid = Grid::new(&self.gl, 20, 11);

        let mut f = file::new("simple-4x4.png");
        let mut texture_loaded = self.texture_stuff(&mut f);

        let mut mousepos = (0.0, 0.0);

        let app = self.app.take().unwrap();
        app.run(move |app: &mut uni_app::App| {
            if !texture_loaded {
                texture_loaded = self.texture_stuff(&mut f);
            }

            self.gl.clear_color(0.5, 0.5, 0.5, 1.0);
            self.gl.clear(uni_gl::BufferBit::Color);

            for i in app.events.borrow().iter() {
                if let uni_app::AppEvent::MousePos(ref pos) = i {
                    mousepos = *pos;
                }
            }

            grid.update_mouse((
                mousepos.0 / f64::from(self.width),
                mousepos.1 / f64::from(self.height),
            ));

            grid.draw(&self.gl);
        });
    }
}
