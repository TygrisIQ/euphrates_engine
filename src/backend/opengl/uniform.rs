pub mod uniform {

    extern crate gl;
    use std::ffi::CString;

    pub fn uniform_location(shader_program: u32, attr_name: CString) -> i32 {
        unsafe { gl::GetUniformLocation(shader_program, attr_name.as_ptr()) }
    }
    pub fn uniform_4f(location: i32, f1: f32, f2: f32, f3: f32, f4: f32) {
        unsafe {
            gl::Uniform4f(location, f1, f2, f3, f4);
        }
    }
}
