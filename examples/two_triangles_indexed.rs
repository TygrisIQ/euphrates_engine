use std::ptr;

use euphrates_engine::{backend, EupWindow};

fn main() {
    use backend::opengl::shader;
    use euphrates_engine::fs;
    //indicies are the order in which the verticies are drawn
    let indicies = [0, 1, 2, 3, 4, 2];
    let verticies: [f32; 15] = [
        -0.5, -0.5, 0.0, //left side
        -0.5, 0.5, 0.0, //top side
        0.0, 0.0, 0.0, //right side
        0.5, 0.5, 0.0, //
        0.5, -0.5, 0.0, //
    ];

    let mut window = EupWindow::new("TRIANGLE...", 640, 480, (3, 3));
    window.load_gl();
    use backend::opengl::globject;
    let mut vao = globject::VAO::new();
    let mut vbo = globject::VBO::new();
    let ebo = globject::EBO::new();
    // bind the vao first so its stores vbo and ebo data, so they can be unbound safely later
    vao.bind();

    vbo.bind();
    ebo.bind();
    globject::vertex_attrib_pointer(0, 3, gl::FALSE, gl::FLOAT, 3, ptr::null());
    globject::upload_data_f32(globject::BufferType::Vbo, &verticies);
    globject::upload_data_i32(globject::BufferType::Ebo, &indicies);
    //ebo.unbind();
    vbo.unbind();
    let loaded_shader = fs::file::load_shader("shaders/triangle_vertex.glsl").unwrap();
    dbg!(&loaded_shader);
    let shader_final = fs::file::shader_to_cstring(loaded_shader).unwrap();
    let vertex_shader =
        shader::ShaderHandle::new_and_compile(&shader_final, shader::ShaderType::Vertex);
    vertex_shader.check_compile_status();
    let fragment_loaded_shader = fs::file::load_shader("shaders/triangle_fragment.glsl").unwrap();
    dbg!(&fragment_loaded_shader);
    let shader_final = fs::file::shader_to_cstring(fragment_loaded_shader).unwrap();
    let fragment_shader =
        shader::ShaderHandle::new_and_compile(&shader_final, shader::ShaderType::Fragment);
    fragment_shader.check_compile_status();
    let shader_program = shader::ShaderProgram::new();
    shader_program.attach_shader(vertex_shader.shader);
    shader_program.attach_shader(fragment_shader.shader);
    shader_program.link_program();
    shader_program.check_link_status();
    fragment_shader.delete_shader();
    vertex_shader.delete_shader();

    println!("PROGRAM ID : {}", shader_program.program_id());
    while !window.should_close() {
        unsafe {
            //dbg!(gl::GetError());
            gl::ClearColor(0.1, 0.2, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            shader_program.use_program();
            vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            //gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.update();
    }
    unsafe {
        gl::DeleteProgram(gl::PROGRAM);
    }
}
