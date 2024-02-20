use std::ffi::c_void;

use gl::{self, types::GLenum};
pub enum TextureWrap {
    S = gl::TEXTURE_WRAP_S as isize,
    T = gl::TEXTURE_WRAP_T as isize,
}
pub enum TextureFiltering {
    NEAREST,
    LINEAR,
}
pub struct TextureHandle(u32);
pub fn texture_parameter_2d() {
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    }
}

pub fn texture_filter_2d() {
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
    }
}
pub fn generate_texture(h: i32, w: i32, data: &[u8]) {
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB8 as i32,
            w,
            h,
            0,
            gl::RGB8,
            gl::UNSIGNED_BYTE,
            data[0] as *const u8 as *const c_void,
        )
    }
}
impl TextureHandle {
    pub fn new() -> TextureHandle {
        unsafe {
            let mut texture: u32 = 0;
            gl::GenTextures(1, &mut texture);

            return TextureHandle(texture);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.0);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
pub fn generate_mipmap() {
    unsafe {
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
}
