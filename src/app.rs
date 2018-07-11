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

    pub fn run(mut self) {
        let fs = include_str!("simple_quad_fs.glsl");
        let vs = include_str!("simple_quad_vs.glsl");

        let program = program::create_program(&self.gl, vs, fs);

        let pos = self.gl.get_attrib_location(&program, "vert_pos").unwrap();
        let tex = self.gl.get_attrib_location(&program, "vert_tex").unwrap();

        let quad = quad::Quad::new(&self.gl, program, pos, tex);

        let app = self.app.take().unwrap();
        app.run(move |_app: &mut uni_app::App| {
            self.gl.clear_color(0.5, 0.5, 0.5, 1.0);
            self.gl.clear(webgl::BufferBit::Color);

            quad.draw(&self.gl);
        });
    }
}
