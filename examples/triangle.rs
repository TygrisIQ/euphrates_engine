use euphrates_engine::{backend, EupWindow};

fn main() {
    use backend::opengl::shader;
    use backend::{opengl, window};
    use euphrates_engine::fs;
    let verticies: [f32; 9] = [
        0.5, -0.5, 0.0, //left side
        0.0, 0.5, 0.0, //top side
        0.5, -0.5, 0.0, //right side
    ];

    let mut window = EupWindow::new("TRIANGLE...", 640, 480, (3, 3));
    window.load_gl();
    use opengl::globject;
    let mut vao = globject::VAO::new();
    let mut vbo = globject::VBO::new();
    vao.bind();
    vbo.bind();
    globject::upload_data_f32(globject::BufferType::Vbo, &verticies);
    vbo.unbind();
    let loaded_shader = fs::fs::load_shader("shaders/triangle_vertex.glsl").unwrap();
    let shader_final = fs::fs::shader_to_cstring(loaded_shader).unwrap();
    let vertex_shader =
        shader::ShaderHandle::create_shader(&shader_final, shader::ShaderType::vertex);
    vertex_shader.check_compile_status();
    let loaded_shader = fs::fs::load_shader("shaders/triangle_fragment.glsl").unwrap();
    let shader_final = fs::fs::shader_to_cstring(loaded_shader).unwrap();
    let fragment_shader =
        shader::ShaderHandle::create_shader(&shader_final, shader::ShaderType::fragment);
    fragment_shader.check_compile_status();
    let shader_program = shader::ShaderProgram::new();
    shader_program.attach_shader(vertex_shader.shader);
    shader_program.attach_shader(fragment_shader.shader);
    shader_program.link_program();
    fragment_shader.delete_shader();
    vertex_shader.delete_shader();
    globject::vertex_attrib_pointer(0, 3, gl::FALSE, gl::FLOAT, 3);
    while !window.should_close() {
        unsafe {
            dbg!(gl::GetError());
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            shader_program.use_program();
            vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.update();
    }
}
