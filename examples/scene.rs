use std::{ffi::CString, mem};
use std::ffi::c_void;

use euphrates_engine::{
    backend::{
        camera::camera::{Camera, CameraMovement},
        opengl::{
            globject::{
                upload_data_f32, upload_data_i32, vertex_attrib_pointer, BufferType, EBO, VAO, VBO,
            },
            shader::{ShaderHandle, ShaderProgram, ShaderType},
            texture::{
                self, generate_mipmap, generate_texture, generate_texture_rgba, texture_filter_2d,
                texture_parameter_2d, TextureHandle,
            },
        },
    },
    fs,
    EupWindow,
};

use gl::types::GLfloat;
use glam::{Mat4, Vec3};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const GL_MAJOR: u32 = 3;
const GL_MINOR: u32 = 3;
const SKY_COLOR: (f32, f32, f32, f32) = (0.07, 0.07, 0.13, 1.0);
const ATTRIB_POS: u32 = 0;
const ATTRIB_UV: u32 = 1;
const STRIDE_FLOATS: usize = 5;

fn main() {
    let mut window = EupWindow::new(
        "euphrates_engine — showcase", WIDTH, HEIGHT, (GL_MAJOR, GL_MINOR));
    window.load_gl();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let (mut cube_vao, _cube_vbo) = create_cube_vao();
    let (mut ground_vao, _ground_vbo, _ground_ebo) = create_ground_vao();

    let _tex_wall      = load_rgba_texture("texture/wall.jpg",      0);
    let _tex_container = load_rgba_texture("texture/container.jpg", 1);
    let _tex_face      = load_rgb_texture("texture/face.png",      2);
    let _tex_grass     = load_rgba_texture("texture/grass.jpg",     3);

    let shader = build_shaders();
    shader.use_program();
    let uniforms = Uniforms::new(shader.program_id());

    let containers: &[(Vec3, f32, f32, i32)] = &[
        (Vec3::new( 1.5, 0.5,  0.0),  20.0, 1.0, 1),
        (Vec3::new(-1.5, 0.5, -1.0), -15.0, 1.0, 1),
        (Vec3::new( 3.0, 0.5, -2.0),  45.0, 1.0, 1),
        (Vec3::new(-2.5, 0.5,  1.5),   0.0, 1.0, 1),
        (Vec3::new( 0.0, 0.5, -3.5),  30.0, 1.0, 1),
        (Vec3::new( 1.5, 1.5,  0.0),  55.0, 0.6, 2),
    ];

    let grass_pos   = Vec3::new(0.0, 1.0, 0.5);
    let grass_scale = 1.0f32;

    let mut camera = Camera::new(Vec3::new(-0.5, 1.2, 5.0));
    camera.yaw   = -95.0;
    camera.pitch = -8.0;
    camera.process_mouse(0.0, 0.0);

    let mut last_time = window.glfw_handle.get_time() as f32;

    while !window.should_close() {
        let now = window.glfw_handle.get_time() as f32;
        let dt  = now - last_time;
        last_time = now;

        camera.process_mouse(-3.0 * dt, 0.0);

        unsafe {
            gl::ClearColor(SKY_COLOR.0, SKY_COLOR.1, SKY_COLOR.2, SKY_COLOR.3);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let view = camera.view_matrix();
        let proj = camera.projection_matrix(WIDTH as f32, HEIGHT as f32);

        shader.use_program();
        uniforms.set_view_projection(&view, &proj);

        uniforms.set_model(&Mat4::IDENTITY);
        uniforms.set_texture_unit(0);
        uniforms.set_tint(1.0, 1.0, 1.0, 1.0);
        unsafe {
            ground_vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        cube_vao.bind();
        for (pos, rot_y_deg, scale, tex_unit) in containers {
            let model =
                Mat4::from_translation(*pos)
                * Mat4::from_rotation_y(rot_y_deg.to_radians())
                * Mat4::from_scale(Vec3::splat(*scale));
            uniforms.set_model(&model);
            uniforms.set_texture_unit(*tex_unit);
            uniforms.set_tint(1.0, 1.0, 1.0, 1.0);
            unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 36); }
        }

        // cube_vao.bind();
        // texture::active_texture(3);
        // _tex_grass.bind();
        // let grass_model =
        //     Mat4::from_translation(grass_pos)
        //     * Mat4::from_rotation_y((now * 40.0).to_radians())
        //     * Mat4::from_scale(Vec3::splat(grass_scale));
        // uniforms.set_model(&grass_model);
        // uniforms.set_texture_unit(3);
        // uniforms.set_tint(1.0, 1.0, 1.0, 1.0);
        // unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 36); 
        //}

        for (_, ev) in glfw::flush_messages(&window.event_handle) {
            match ev {
                glfw::WindowEvent::Key(glfw::Key::W, _, glfw::Action::Press | glfw::Action::Repeat, _) =>
                    camera.process_keyboard(CameraMovement::Forward, dt),
                glfw::WindowEvent::Key(glfw::Key::S, _, glfw::Action::Press | glfw::Action::Repeat, _) =>
                    camera.process_keyboard(CameraMovement::Backward, dt),
                glfw::WindowEvent::Key(glfw::Key::A, _, glfw::Action::Press | glfw::Action::Repeat, _) =>
                    camera.process_keyboard(CameraMovement::Left, dt),
                glfw::WindowEvent::Key(glfw::Key::D, _, glfw::Action::Press | glfw::Action::Repeat, _) =>
                    camera.process_keyboard(CameraMovement::Right, dt),
                _ => {}
            }
        }

        window.update();
    }
}

struct Uniforms {
    u_model: i32,
    u_view:  i32,
    u_proj:  i32,
    u_tex:   i32,
    u_tint:  i32,
}

impl Uniforms {
    fn new(program: u32) -> Self {
        unsafe {
            Self {
                u_model: gl::GetUniformLocation(program, cstr("model").as_ptr()),
                u_view:  gl::GetUniformLocation(program, cstr("view").as_ptr()),
                u_proj:  gl::GetUniformLocation(program, cstr("projection").as_ptr()),
                u_tex:   gl::GetUniformLocation(program, cstr("tex").as_ptr()),
                u_tint:  gl::GetUniformLocation(program, cstr("tint").as_ptr()),
            }
        }
    }

    fn set_model(&self, m: &Mat4) {
        unsafe { gl::UniformMatrix4fv(self.u_model, 1, gl::FALSE, m.as_ref().as_ptr()); }
    }

    fn set_view_projection(&self, view: &Mat4, proj: &Mat4) {
        unsafe {
            gl::UniformMatrix4fv(self.u_view, 1, gl::FALSE, view.as_ref().as_ptr());
            gl::UniformMatrix4fv(self.u_proj, 1, gl::FALSE, proj.as_ref().as_ptr());
        }
    }

    fn set_texture_unit(&self, unit: i32) {
        unsafe { gl::Uniform1i(self.u_tex, unit); }
    }

    fn set_tint(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { gl::Uniform4f(self.u_tint, r, g, b, a); }
    }
}

fn create_cube_vao() -> (VAO, VBO) {
    let verts: [f32; 180] = [
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
    upload_data_f32(BufferType::Vbo, &verts);
    vertex_attrib_pointer(ATTRIB_POS, 3, gl::FALSE, gl::FLOAT, STRIDE_FLOATS, std::ptr::null());
    vertex_attrib_pointer(ATTRIB_UV,  2, gl::FALSE, gl::FLOAT, STRIDE_FLOATS,
        (3 * mem::size_of::<GLfloat>()) as *const c_void);
    (vao, vbo)
}

fn create_ground_vao() -> (VAO, VBO, EBO) {
    let verts: [f32; 20] = [
        -8.0, 0.0, -8.0,  0.0, 0.0,
         8.0, 0.0, -8.0,  8.0, 0.0,
         8.0, 0.0,  8.0,  8.0, 8.0,
        -8.0, 0.0,  8.0,  0.0, 8.0,
    ];
    let idx: [i32; 6] = [0, 1, 2, 2, 3, 0];

    let mut vao = VAO::new();
    let mut vbo = VBO::new();
    let ebo = EBO::new();
    vao.bind();
    vbo.bind();
    ebo.bind();
    upload_data_i32(BufferType::Ebo, &idx);
    upload_data_f32(BufferType::Vbo, &verts);
    vertex_attrib_pointer(ATTRIB_POS, 3, gl::FALSE, gl::FLOAT, STRIDE_FLOATS, std::ptr::null());
    vertex_attrib_pointer(ATTRIB_UV,  2, gl::FALSE, gl::FLOAT, STRIDE_FLOATS,
        (3 * mem::size_of::<GLfloat>()) as *const c_void);
    (vao, vbo, ebo)
}
fn load_rgb_texture(path: &str, unit: u8) -> TextureHandle {
    let t = TextureHandle::new();
    texture::active_texture(unit);
    t.bind();
    texture_parameter_2d();
    texture_filter_2d();
    let img = fs::image_handle::load_image(path);
    let px  = fs::image_handle::image_pixels_rgb(&img);
    use image::GenericImageView;
    generate_texture(img.width() as i32, img.height() as i32, &px);
    generate_mipmap();
    t
}
fn load_rgba_texture(path: &str, unit: u8) -> TextureHandle {
    let tex = TextureHandle::new();
    texture::active_texture(unit);
    tex.bind();
    texture_parameter_2d();
    texture_filter_2d();
    let img = fs::image_handle::load_image(path);
    let px  = fs::image_handle::image_pixels_rgba(&img);
    generate_texture_rgba(img.width() as i32, img.height() as i32, &px);
    generate_mipmap();
    tex
}

fn build_shaders() -> ShaderProgram {
    let vs_src = fs::file::shader_to_cstring(fs::file::load_shader("shaders/camera_vs.glsl").unwrap()).unwrap();
    let fs_src = fs::file::shader_to_cstring(fs::file::load_shader("shaders/camera_fs.glsl").unwrap()).unwrap();
    let v = ShaderHandle::new_and_compile(&vs_src, ShaderType::Vertex);
    v.check_compile_status();
    let f = ShaderHandle::new_and_compile(&fs_src, ShaderType::Fragment);
    f.check_compile_status();
    let p = ShaderProgram::new();
    p.attach_shader(v.shader);
    p.attach_shader(f.shader);
    p.link_program();
    p.check_link_status();
    v.delete_shader();
    f.delete_shader();
    p
}

fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}