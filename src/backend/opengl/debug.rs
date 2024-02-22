use gl::types::{GLchar, GLenum, GLsizei, GLuint};

/// Wrapper for the DebugMessageCallback
/// requires opengl 4.4 to be supported by the gpu

pub extern "system" fn gl_debug_callback_wrapper(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    _user_param: *mut std::ffi::c_void,
) {
    gl_debug_callback(source, type_, id, severity, length, message, _user_param);
}
#[allow(unused)]
fn gl_debug_callback(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    _user_param: *mut std::ffi::c_void,
) {
    unsafe {
        println!(
            "DEBUG MESSAGE:  {} \n\n\n",
            std::ffi::CStr::from_ptr(message).to_string_lossy()
        );
    }
}
