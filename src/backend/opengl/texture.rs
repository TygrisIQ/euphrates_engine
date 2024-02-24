use std::ffi::c_void;

use gl;

pub struct TextureHandle(pub u32);
/// wrapper around the `gl::TexParameteri()`, wraps textures S,T
pub fn texture_parameter_2d() {
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    }
}
/// wrapper around `gl::TexParameteri()`, for setting up 2D texture filtering
pub fn texture_filter_2d() {
    unsafe {
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }
}
/// calls 'glTexImage2D' with the provided arguments
pub fn generate_texture(w: i32, h: i32, data: &[u8]) {
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            w,
            h,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        )
    }
}
pub fn generate_texture_rgba(w: i32, h: i32, data: &[u8]) {
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            w,
            h,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        )
    }
}
pub fn active_texture(current: u8) {
    let active = match current {
        0 => gl::TEXTURE0,
        1 => gl::TEXTURE1,
        2 => gl::TEXTURE2,
        _ => gl::TEXTURE0,
    };
    unsafe {
        gl::ActiveTexture(active);
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
/// calls `glGenerateMipmap` for texture 2D
pub fn generate_mipmap() {
    unsafe {
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
}
