use std::ffi::CString;
use euphrates_engine::{
    backend::{
        opengl::{
            globject::{upload_data_f32, upload_data_i32, vertex_attrib_pointer, EBO, VAO, VBO, BufferType},
            shader::{ShaderHandle, ShaderProgram},
            texture::{self, generate_texture, generate_mipmap, texture_filter_2d, texture_parameter_2d, TextureHandle},
        },
        camera::camera::{Camera, CameraMovement},
    },
    fs,
    EupWindow,
};
use gl::types::GLfloat;
use glam::{Mat4, Vec3};
use std::mem;
use std::ffi::c_void;

fn main() {
    let mut window = EupWindow::new("Camera", 800, 600, (3, 3));
    window.load_gl();

    // A cube: 36 vertices (6 faces * 2 triangles * 3 verts), pos + texcoord
    let vertices: [f32; 180] = [
        -0.5,-0.5,-0.5, 0.0,0.0,  0.5,-0.5,-0.5, 1.0,0.0,  0.5, 0.5,-0.5, 1.0,1.0,
         0.5, 0.5,-0.5, 1.0,1.0, -0.5, 0.5,-0.5, 0.0,1.0, -0.5,-0.5,-0.5, 0.0,0.0,
        -0.5,-0.5, 0.5, 0.0,0.0,  0.5,-0.5, 0.5, 1.0,0.0,  0.5, 0.5, 0.5, 1.0,1.0,
         0.5, 0.5, 0.5, 1.0,1.0, -0.5, 0.5, 0.5, 0.0,1.0, -0.5,-0.5, 0.5, 0.0,0.0,
        -0.5, 0.5, 0.5, 1.0,0.0, -0.5, 0.5,-0.5, 1.0,1.0, -0.5,-0.5,-0.5, 0.0,1.0,
        -0.5,-0.5,-0.5, 0.0,1.0, -0.5,-0.5, 0.5, 0.0,0.0, -0.5, 0.5, 0.5, 1.0,0.0,
         0.5, 0.5, 0.5, 1.0,0.0,  0.5, 0.5,-0.5, 1.0,1.0,  0.5,-0.5,-0.5, 0.0,1.0,
         0.5,-0.5,-0.5, 0.0,1.0,  0.5,-0.5, 0.5, 0.0,0.0,  0.5, 0.5, 0.5, 1.0,0.0,
        -0.5,-0.5,-0.5, 0.0,1.0,  0.5,-0.5,-0.5, 1.0,1.0,  0.5,-0.5, 0.5, 1.0,0.0,
         0.5,-0.5, 0.5, 1.0,0.0, -0.5,-0.5, 0.5, 0.0,0.0, -0.5,-0.5,-0.5, 0.0,1.0,
        -0.5, 0.5,-0.5, 0.0,1.0,  0.5, 0.5,-0.5, 1.0,1.0,  0.5, 0.5, 0.5, 1.0,0.0,
         0.5, 0.5, 0.5, 1.0,0.0, -0.5, 0.5, 0.5, 0.0,0.0, -0.5, 0.5,-0.5, 0.0,1.0,
    ];

    let mut vao = VAO::new();
    let mut vbo = VBO::new();
    vao.bind();
    vbo.bind();
    upload_data_f32(BufferType::Vbo, &vertices);
    vertex_attrib_pointer(0, 3, gl::FALSE, gl::FLOAT, 5, std::ptr::null());
    vertex_attrib_pointer(1, 2, gl::FALSE, gl::FLOAT, 5,
        (3 * mem::size_of::<GLfloat>()) as *const c_void);

    // load texture
    let texture = TextureHandle::new();
    texture::active_texture(0);
    texture.bind();
    texture_parameter_2d();
    texture_filter_2d();
    let img = fs::image_handle::load_image("texture/wall.jpg");
    let pixels = fs::image_handle::image_pixels_rgb(&img);
    use image::GenericImageView;
    generate_texture(img.width() as i32, img.height() as i32, &pixels);
    generate_mipmap();

    let shader = build_shaders();
    shader.use_program();
    // bind texture unit
    let tex1 = CString::new("texture1").unwrap();
    unsafe {
        gl::Uniform1i(gl::GetUniformLocation(shader.program_id(), tex1.as_ptr()), 0);
    }

    // uniform locations
    let u_model      = CString::new("model").unwrap();
    let u_view       = CString::new("view").unwrap();
    let u_projection = CString::new("projection").unwrap();

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 3.0));
    let mut last_time = window.glfw_handle.get_time() as f32;

    unsafe { gl::Enable(gl::DEPTH_TEST); }

    while !window.should_close() {
        let now = window.glfw_handle.get_time() as f32;
        let delta = now - last_time;
        last_time = now;

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // handle input
        for (_, ev) in glfw::flush_messages(&window.event_handle) {
            match ev {
                glfw::WindowEvent::Key(glfw::Key::W, _, glfw::Action::Press | glfw::Action::Repeat, _) =>
                    camera.process_keyboard(CameraMovement::Forward, delta),
                glfw::WindowEvent::Key(glfw::Key::S, _, glfw::Action::Press | glfw::Action::Repeat, _) =>
                    camera.process_keyboard(CameraMovement::Backward, delta),
                glfw::WindowEvent::Key(glfw::Key::A, _, glfw::Action::Press | glfw::Action::Repeat, _) =>
                    camera.process_keyboard(CameraMovement::Left, delta),
                glfw::WindowEvent::Key(glfw::Key::D, _, glfw::Action::Press | glfw::Action::Repeat, _) =>
                    camera.process_keyboard(CameraMovement::Right, delta),
                _ => {}
            }
        }

        let model      = Mat4::IDENTITY;
        let view       = camera.view_matrix();
        let projection = camera.projection_matrix(800.0, 600.0);

        unsafe {
            shader.use_program();
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(shader.program_id(), u_model.as_ptr()),
                1, gl::FALSE, model.as_ref().as_ptr());
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(shader.program_id(), u_view.as_ptr()),
                1, gl::FALSE, view.as_ref().as_ptr());
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(shader.program_id(), u_projection.as_ptr()),
                1, gl::FALSE, projection.as_ref().as_ptr());

            vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        window.update();
    }
}

fn build_shaders() -> ShaderProgram {
    let vs_src = fs::file::shader_to_cstring(
        fs::file::load_shader("shaders/camera_vs.glsl").unwrap()).unwrap();
    let fs_src = fs::file::shader_to_cstring(
        fs::file::load_shader("shaders/camera_fs.glsl").unwrap()).unwrap();
    use euphrates_engine::backend::opengl::shader::{ShaderHandle, ShaderType};
    let vs = ShaderHandle::new_and_compile(&vs_src, ShaderType::Vertex);
    vs.check_compile_status();
    let fs = ShaderHandle::new_and_compile(&fs_src, ShaderType::Fragment);
    fs.check_compile_status();
    let program = ShaderProgram::new();
    program.attach_shader(vs.shader);
    program.attach_shader(fs.shader);
    program.link_program();
    program.check_link_status();
    vs.delete_shader();
    fs.delete_shader();
    program
}