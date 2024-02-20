use std::{ffi::c_void, mem, ptr};

use euphrates_engine::{
    backend::{
        self,
        opengl::{
            globject::{self, upload_data_f32, upload_data_i32, vertex_attrib_pointer},
            shader::{self, ShaderHandle},
            texture::{
                generate_mipmap, generate_texture, texture_filter_2d, texture_parameter_2d,
                TextureFiltering, TextureHandle,
            },
        },
        window::eupwindow::eup_window,
    },
    fs::fs::{self, shader_to_cstring},
};
use gl::types::GLfloat;
use image::GenericImageView;

fn main() {
    use euphrates_engine::fs::image_handle;
    //dbg!(load_image_pixels("texture/wall.jpg").unwrap());
    let verticies: [f32; 32] = [
        -0.5, -0.5, 0.0, //vertex 1
        1.0, 0.0, 0.0, //vertex 1 color
        0.0, 0.0, // vertex 1 texture coords
        -0.5, 0.5, 0.0, //vertex 2 pos
        0.0, 1.0, 0.0, //vertex 2 color
        0.0, 1.0, // vertex 2 texture coords
        0.5, -0.5, 0.0, //vertex 3 pos
        0.0, 0.0, 1.0, //vertex 3 color
        1.0, 0.0, //vertex 3 texture coords
        0.5, 0.5, 0.0, // v 4 pos
        1.0, 1.0, 1.0, //
        1.0, 1.0,
    ];
    let indicies = [0, 1, 3, 0, 3, 2];
    let mut window = eup_window::EupWindow::new("TEXTURE.", 640, 480, (3, 3));
    window.load_gl();
    let vs_s = fs::load_shader("shaders/tex_vs.glsl").unwrap();
    let vs_c = shader_to_cstring(vs_s).unwrap();
    let fs_s = fs::load_shader("shaders/tex_fs.glsl").unwrap();
    let fs_c = shader_to_cstring(fs_s).unwrap();
    let vs = ShaderHandle::create_shader(&vs_c, shader::ShaderType::Vertex);
    let fs = ShaderHandle::create_shader(&fs_c, shader::ShaderType::Fragment);
    vs.check_compile_status();
    fs.check_compile_status();
    let program = shader::ShaderProgram::new();
    program.attach_shader(vs.shader);
    program.attach_shader(fs.shader);
    program.link_program();
    vs.delete_shader();
    fs.delete_shader();

    let mut vao = globject::VAO::new();
    let mut vbo = globject::VBO::new();
    vao.bind();
    vbo.bind();
    let ebo = globject::EBO::new();
    ebo.bind();
    let tex = TextureHandle::new();
    tex.bind();
    upload_data_i32(globject::BufferType::Ebo, &indicies);
    upload_data_f32(globject::BufferType::Vbo, &verticies);
    //position data
    vertex_attrib_pointer(0, 3, gl::FALSE, gl::FLOAT, 8, ptr::null());
    // color data
    vertex_attrib_pointer(
        1,
        3,
        gl::FALSE,
        gl::FLOAT,
        8,
        //it tells the pointer to jump 3 values when processing each vertex to get to the color
        //data
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );
    //texture coordinates data
    vertex_attrib_pointer(
        2,
        2,
        gl::FALSE,
        gl::FLOAT,
        8,
        (6 * mem::size_of::<GLfloat>()) as *const c_void,
    );
    let img = image_handle::load_image("texture/wall.jpg").to_owned();
    let pixels = image_handle::image_pixels(&img);

    texture_parameter_2d();
    texture_filter_2d();
    generate_texture(img.height() as i32, img.width() as i32, &pixels);
    generate_mipmap();
    //vbo.unbind();
    //ebo.unbind();
    program.check_link_status();
    while !window.should_close() {
        unsafe {
            window.update();

            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            vao.bind();
            tex.bind();
            program.use_program();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, 0);
        gl::DeleteProgram(program.program_id());
    }
}
