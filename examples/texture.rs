use std::{ffi::c_void, mem};

use euphrates_engine::{
    backend::{
        self,
        opengl::{
            globject::{
                self, upload_data_f32, upload_data_i32, vertex_attrib_pointer, EBO, VAO, VBO,
            },
            shader::{ShaderHandle, ShaderProgram},
        },
        *,
    },
    fs, EupWindow,
};
use gl::{types::GLfloat, DrawElements, ShaderSource};
#[allow(dead_code)]
fn main() {
    let mut window = EupWindow::new("TEXTURE.", 640, 480, (3, 3));
    window.load_gl();
    // first 3 values are position of the vertex and the next 2 are the texture coordinates
    let verticies: [f32; 20] = [
        -0.5, -0.5, 0.0, //v1 pos
        0.0, 0.0, //v1 tex
        -0.5, 0.5, 0.0, //v2 position
        0.0, 1.0, //v2 tex
        0.5, 0.5, 0.0, //v3 position
        1.0, 1.0, //v3 tex coords
        0.5, -0.5, 0.0, //v4 position
        1.0, 0.0,
    ];
    //indicies are the order in which verticies are drawn, im trying to draw a rectangle with 4
    //verticies (points) so i will draw 2 triangles, this is so i dont forget...
    let indicies = [0, 1, 3, 1, 3, 2];
    let shader_program = do_shaders();
    let mut vao = VAO::new();
    let mut vbo = VBO::new();
    let mut ebo = EBO::new();
    vao.bind();
    vbo.bind();
    vertex_attrib_pointer(0, 3, gl::FALSE, gl::FLOAT, 5, std::ptr::null());
    vertex_attrib_pointer(
        1,
        2,
        gl::FALSE,
        gl::FLOAT,
        5,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );
    ebo.bind();
    upload_data_i32(globject::BufferType::Ebo, &indicies);
    upload_data_f32(globject::BufferType::Vbo, &verticies);
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(
            Some(backend::opengl::debug::gl_debug_callback_wrapper),
            std::ptr::null(),
        );
    }
    let tex = unsafe {
        let mut tex: u32 = 0;
        gl::GenTextures(1, &mut tex);
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        let img = fs::image_handle::load_image("texture/wall.jpg");
        let pixels = fs::image_handle::image_pixels(&img);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &pixels[0] as *const u8 as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        tex
    };
    vao.unbind();
    vbo.unbind();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            shader_program.use_program();
            vao.bind();

            DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
        window.update();
    }
}

fn do_shaders() -> ShaderProgram {
    let shadersource = fs::fs::load_shader("shaders/tex_vs.glsl").unwrap();
    let cstr_vs = fs::fs::shader_to_cstring(shadersource).unwrap();
    let vertex = ShaderHandle::new_and_compile(&cstr_vs, opengl::shader::ShaderType::Vertex);
    vertex.check_compile_status();
    let shadersource = fs::fs::load_shader("shaders/tex_fs.glsl").unwrap();
    let cstr_fs = fs::fs::shader_to_cstring(shadersource).unwrap();
    let fragment = ShaderHandle::new_and_compile(&cstr_fs, opengl::shader::ShaderType::Fragment);
    fragment.check_compile_status();
    let program = ShaderProgram::new();
    program.attach_shader(vertex.shader);
    program.attach_shader(fragment.shader);
    program.link_program();
    program.check_link_status();
    vertex.delete_shader();
    fragment.delete_shader();

    program
}
