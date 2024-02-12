use std::ptr;

use gl::types::{GLchar, GLenum, GLint};
use std::ffi::CString;
extern crate gl;
pub struct ShaderHandle {
    pub shader: u32,
    shader_type: GLenum,
}
#[derive(Debug)]
pub struct ShaderProgram(u32);

pub enum ShaderType {
    Vertex,
    Fragment,
}
#[allow(dead_code)]
impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        unsafe { ShaderProgram(gl::CreateProgram()) }
    }
    pub fn attach_shader(&self, shader: u32) {
        unsafe {
            gl::AttachShader(self.0, shader);
        }
    }
    pub fn program_id(&self) -> u32 {
        self.0
    }
    pub fn link_program(&self) {
        unsafe {
            gl::LinkProgram(self.0);
        }
    }
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }
    pub fn check_link_status(&self) {
        unsafe {
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);
            gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    self.0,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::LINKING::PROGRAM \n {}",
                    std::str::from_utf8_unchecked(&info_log)
                );
            }
        }
    }
}
impl ShaderHandle {
    pub fn create_shader(source: &CString, shader_type: ShaderType) -> ShaderHandle {
        unsafe {
            let shadertype = match shader_type {
                ShaderType::Vertex => gl::VERTEX_SHADER,
                ShaderType::Fragment => gl::FRAGMENT_SHADER,
            };

            let glshader = gl::CreateShader(shadertype);
            gl::ShaderSource(glshader, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(glshader);
            return ShaderHandle {
                shader: glshader,
                shader_type: shadertype,
            };
        }
    }
    pub fn delete_shader(&self) {
        unsafe {
            gl::DeleteShader(self.shader);
        }
    }
    pub fn check_compile_status(&self) {
        unsafe {
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
                dbg!(
                    "ERROR::COMPILING::{}::SHADER \n {}",
                    self.shader_type,
                    std::str::from_utf8_unchecked(&infoLog)
                );
            }
        }
    }
}
