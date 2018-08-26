use std::mem::size_of;
use uni_gl::*;

fn compile_shader(
    gl: &WebGLRenderingContext,
    shader_kind: ShaderKind,
    source: &str,
) -> WebGLShader {
    let shader = gl.create_shader(shader_kind);

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    shader
}

fn compile_shader_wasm_native(
    gl: &WebGLRenderingContext,
    shader_kind: ShaderKind,
    source: &str,
) -> WebGLShader {
    let prefix = if IS_GL_ES {
        "#version 300 es\n"
    } else {
        "#version 150\n"
    }.to_string();

    compile_shader(&gl, shader_kind, &(prefix + source))
}

pub fn create_program(
    gl: &WebGLRenderingContext,
    vertex_source: &str,
    fragment_source: &str,
) -> WebGLProgram {
    let vert_shader = compile_shader_wasm_native(&gl, ShaderKind::Vertex, vertex_source);
    let frag_shader = compile_shader_wasm_native(&gl, ShaderKind::Fragment, fragment_source);

    let program = gl.create_program();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    program
}

pub trait AsDataSlice {
    fn as_data_slice(&self) -> &[u8];
}

impl<T> AsDataSlice for [T] {
    fn as_data_slice(&self) -> &[u8] {
        let len = size_of::<T>() * self.len();

        unsafe {
            use std::slice;
            slice::from_raw_parts(self.as_ptr() as *const u8, len)
        }
    }
}

pub fn set_buffer_data(
    gl: &WebGLRenderingContext,
    buffer: &WebGLBuffer,
    data: &[f32],
    attribute_location: u32,
    count_per_vertex: AttributeSize,
) {
    gl.bind_buffer(BufferKind::Array, buffer);
    gl.enable_vertex_attrib_array(attribute_location);

    gl.buffer_data(BufferKind::Array, data.as_data_slice(), DrawMode::Stream);

    gl.vertex_attrib_pointer(
        attribute_location,
        count_per_vertex,
        DataType::Float,
        false,
        0,
        0,
    );
}
