use std::ptr;

use gl::types::{GLchar, GLenum, GLint};
use std::ffi::CString;
extern crate gl;
struct ShaderHandle {
    shader: u32,
    shader_type: GLenum,
}

struct ShaderProgram(u32);

enum ShaderType {
    vertex,
    fragment,
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        unsafe { ShaderProgram(gl::CreateProgram()) }
    }
    pub fn attach_shader(&self, shader: u32) {
        unsafe {
            gl::AttachShader(self.0, shader);
        }
    }
    pub fn link_program(&self) {
        unsafe {
            gl::LinkProgram(self.0);
        }
    }
    pub fn check_link_status(&self) {
        unsafe {
            use std::str;
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);
            gl::GetShaderiv(self.0, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    self.0,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::LINKING::PROGRAM \n {}",
                    std::str::from_utf8(&info_log).unwrap()
                );
            }
        }
    }
}
impl ShaderHandle {
    pub fn new(source: &CString, shader_type: ShaderType) -> ShaderHandle {
        unsafe {
            let shader_type = match shader_type {
                ShaderType::vertex => gl::VERTEX_SHADER,
                ShaderType::fragment => gl::FRAGMENT_SHADER,
            };

            let glshader = gl::CreateShader(shader_type);
            gl::ShaderSource(glshader, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(glshader);
            return ShaderHandle {
                shader: glshader,
                shader_type: shader_type,
            };
        }
    }
    fn check_compile_status(&self) {
        unsafe {
            use std::str;
            let mut success = gl::FALSE as GLint;
            let mut infoLog = Vec::with_capacity(512);
            infoLog.set_len(512 - 1);
            gl::GetShaderiv(self.shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    self.shader,
                    512,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::COMPILING::{}::SHADER \n {}",
                    self.shader_type,
                    std::str::from_utf8(&infoLog).unwrap()
                );
            }
        }
    }
}
