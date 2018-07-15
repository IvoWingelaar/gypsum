use file;
use grid::Grid;
use image;
use texture;
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
            title: "Title".into(),
            vsync: false,
            show_cursor: true,
            headless: false,
            resizable: false,
            fullscreen: false,
        });

        let gl = webgl::WebGLRenderingContext::new(app.canvas());
        gl.viewport(0, 0, width, height);

        gl.enable(webgl::Flag::Blend as i32);
        gl.blend_equation(webgl::BlendEquation::FuncAdd);
        gl.blend_func(
            webgl::BlendMode::SrcAlpha,
            webgl::BlendMode::OneMinusSrcAlpha,
        );

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

        let texture = texture::new_and_setup(&self.gl);

        let content = file.read_binary().unwrap();
        let im = image::load_from_memory(&content).unwrap().to_rgba();

        self.gl.active_texture(0);
        self.gl.bind_texture(&texture);

        texture::upload_rgba8(&self.gl, im.width() as u16, im.height() as u16, &im);
        true
    }

    pub fn run(mut self) {
        let grid = Grid::new(&self.gl, 200, 100);

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

            grid.draw(&self.gl);
        });
    }
}
