use gl::{
    self,
    types::{GLboolean, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr},
};

//use num_traits::Num;
use std::{ffi::c_void, mem, ptr};
struct VAO(u32);
struct VBO(u32);
struct EBO(u32);

enum BufferType {
    Vbo,
    Ebo,
}
#[allow(dead_code)]
fn vertex_attrib_pointer(
    start_index: u32,
    size: i32,
    normalized: GLboolean,
    attrib_type: GLenum,
    number_of_attributes_per_vertex: usize,
) {
    unsafe {
        let stride = number_of_attributes_per_vertex * std::mem::size_of::<GLfloat>();
        //TODO! solve the pointer issue

        gl::VertexAttribPointer(
            start_index,
            size,
            attrib_type,
            normalized,
            stride as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(start_index);
    }
}

fn upload_data_f32(buffertype: BufferType, data: &[f32]) {
    let size = (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
    let buffer = match buffertype {
        BufferType::Vbo => gl::ARRAY_BUFFER,
        BufferType::Ebo => gl::ELEMENT_ARRAY_BUFFER,
    };

    unsafe {
        gl::BufferData(
            buffer,
            size,
            &data[0] as *const f32 as *const _,
            gl::STATIC_DRAW,
        );
    }
}
fn upload_data_i32(buffertype: BufferType, data: &[i32]) {
    let size = (data.len() * mem::size_of::<GLint>()) as GLsizeiptr;
    let buffer = match buffertype {
        BufferType::Vbo => gl::ARRAY_BUFFER,
        BufferType::Ebo => gl::ELEMENT_ARRAY_BUFFER,
    };

    unsafe {
        gl::BufferData(
            buffer,
            size,
            &data[0] as *const i32 as *const _,
            gl::STATIC_DRAW,
        );
    }
}
impl VBO {
    pub fn new() -> VBO {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        VBO(vbo)
    }

    pub fn delete_buffer(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, self.0 as *const _);
        }
    }
    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.0);
        }
    }
    pub fn unbind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl VAO {
    pub fn new() -> VAO {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        VAO(vao)
    }

    pub fn delete_buffer(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, self.0 as *const _);
        }
    }
    pub fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.0);
        }
    }
    pub fn unbind(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
