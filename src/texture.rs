use uni_gl::{WebGLRenderingContext, WebGLTexture};

pub(crate) fn new_and_setup(gl: &WebGLRenderingContext) -> WebGLTexture {
    let texture = gl.create_texture();
    gl.bind_texture(&texture);

    use uni_gl::TextureKind;
    use uni_gl::TextureParameter;
    use uni_gl::TextureWrap;
    use uni_gl::{TextureMagFilter, TextureMinFilter};

    gl.tex_parameteri(
        TextureKind::Texture2d,
        TextureParameter::TextureMagFilter,
        TextureMagFilter::Nearest as i32,
    );
    gl.tex_parameteri(
        TextureKind::Texture2d,
        TextureParameter::TextureMinFilter,
        TextureMinFilter::Nearest as i32,
    );

    let wrap = TextureWrap::ClampToEdge as i32;

    gl.tex_parameteri(TextureKind::Texture2d, TextureParameter::TextureWrapS, wrap);
    gl.tex_parameteri(TextureKind::Texture2d, TextureParameter::TextureWrapT, wrap);

    texture
}

pub(crate) fn upload_rgba8(gl: &WebGLRenderingContext, width: u16, height: u16, data: &[u8]) {
    use uni_gl::{PixelFormat, PixelType, TextureBindPoint};

    gl.tex_image2d(
        TextureBindPoint::Texture2d,
        0,
        width,
        height,
        PixelFormat::Rgba,
        PixelType::UnsignedByte,
        data,
    );
}

pub(crate) fn upload_sub_rgba8(gl: &WebGLRenderingContext, width: u16, height: u16, data: &[u8]) {
    use uni_gl::{PixelFormat, PixelType, TextureBindPoint};

    gl.tex_sub_image2d(
        TextureBindPoint::Texture2d,
        0,
        0,
        0,
        width,
        height,
        PixelFormat::Rgba,
        PixelType::UnsignedByte,
        data,
    );
}
