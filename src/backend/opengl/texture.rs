use gl::{self, types::GLenum};
pub enum TextureWrap {
    S = gl::TEXTURE_WRAP_S as isize,
    T = gl::TEXTURE_WRAP_T as isize,
}
//P
pub fn texture_parameter_2d(wrap_type: TextureWrap) {
    unsafe {
        gl::TexParameteri(
            gl::TEXTURE_2D,
            wrap_type as GLenum,
            gl::MIRRORED_REPEAT as i32,
        );
    }
}
