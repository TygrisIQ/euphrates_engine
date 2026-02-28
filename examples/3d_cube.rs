// use std::{ffi::c_void, mem, ptr};
// use cgmath::{Matrix4, Point3, Vector3, perspective, Deg, InnerSpace};
// use euphrates_engine::{
//     backend::{
//         self,
//         opengl::{
//             globject::{self, upload_data_f32, VAO, VBO},
//             shader::{ShaderHandle, ShaderProgram},
//             texture::{self, TextureHandle},
//             uniform,
//         },
//         window::eupwindow::eup_window::EupWindow,
//     },
//     fs::{self},
// };
// use gl;

// fn main() {
//     let mut window = EupWindow::new("3D Camera", 800, 600, (3, 3));
//     window.load_gl();

//     let mut camera = Camera::new(
//         Point3::new(0.0, 0.0, 3.0),
//         Vector3::new(0.0, 1.0, 0.0),
//         -90.0,
//         0.0,
//     );

//     let vertices: [f32; 180] = [
//         -0.5, -0.5, -0.5,  0.0, 0.0,
//          0.5, -0.5, -0.5,  1.0, 0.0,
//          0.5,  0.5, -0.5,  1.0, 1.0,
//          0.5,  0.5, -0.5,  1.0, 1.0,
//         -0.5,  0.5, -0.5,  0.0, 1.0,
//         -0.5, -0.5, -0.5,  0.0, 0.0,

//         -0.5, -0.5,  0.5,  0.0, 0.0,
//          0.5, -0.5,  0.5,  1.0, 0.0,
//          0.5,  0.5,  0.5,  1.0, 1.0,
//          0.5,  0.5,  0.5,  1.0, 1.0,
//         -0.5,  0.5,  0.5,  0.0, 1.0,
//         -0.5, -0.5,  0.5,  0.0, 0.0,

//         -0.5,  0.5,  0.5,  1.0, 0.0,
//         -0.5,  0.5, -0.5,  1.0, 1.0,
//         -0.5, -0.5, -0.5,  0.0, 1.0,
//         -0.5, -0.5, -0.5,  0.0, 1.0,
//         -0.5, -0.5,  0.5,  0.0, 0.0,
//         -0.5,  0.5,  0.5,  1.0, 0.0,

//          0.5,  0.5,  0.5,  1.0, 0.0,
//          0.5,  0.5, -0.5,  1.0, 1.0,
//          0.5, -0.5, -0.5,  0.0, 1.0,
//          0.5, -0.5, -0.5,  0.0, 1.0,
//          0.5, -0.5,  0.5,  0.0, 0.0,
//          0.5,  0.5,  0.5,  1.0, 0.0,

//         -0.5, -0.5, -0.5,  0.0, 1.0,
//          0.5, -0.5, -0.5,  1.0, 1.0,
//          0.5, -0.5,  0.5,  1.0, 0.0,
//          0.5, -0.5,  0.5,  1.0, 0.0,
//         -0.5, -0.5,  0.5,  0.0, 0.0,
//         -0.5, -0.5, -0.5,  0.0, 1.0,

//         -0.5,  0.5, -0.5,  0.0, 1.0,
//          0.5,  0.5, -0.5,  1.0, 1.0,
//          0.5,  0.5,  0.5,  1.0, 0.0,
//          0.5,  0.5,  0.5,  1.0, 0.0,
//         -0.5,  0.5,  0.5,  0.0, 0.0,
//         -0.5,  0.5, -0.5,  0.0, 1.0
//     ];

//     let mut vao = VAO::new();
//     let mut vbo = VBO::new();
    
//     vao.bind();
//     vbo.bind();
    
//     upload_data_f32(globject::BufferType::Vbo, &vertices);
    
//     globject::vertex_attrib_pointer(0, 3, gl::FALSE, gl::FLOAT, 5, ptr::null());
//     globject::vertex_attrib_pointer(1, 2, gl::FALSE, gl::FLOAT, 5, (3 * mem::size_of::<f32>()) as *const c_void);

//     let program = load_shaders("shaders/camera_vs.glsl", "shaders/camera_fs.glsl");
    
//     let texture = TextureHandle::new();
//     texture::active_texture(0);
//     texture.bind();
//     texture::texture_parameter_2d();
//     texture::texture_filter_2d();
    
//     let img = fs::image_handle::load_image("texture/container.jpg");
//     let pixels = fs::image_handle::image_pixels_rgb(&img);
//     texture::generate_texture(img.width() as i32, img.height() as i32, &pixels);
//     texture::generate_mipmap();

//     unsafe { gl::Enable(gl::DEPTH_TEST); }

//     let mut last_frame = 0.0;
    
//     while !window.should_close() {
//         let current_frame = window.glfw_handle.get_time() as f32;
//         let delta_time = current_frame - last_frame;
//         last_frame = current_frame;

//         unsafe {
//             gl::ClearColor(0.1, 0.1, 0.1, 1.0);
//             gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
//             program.use_program();
            
//             let projection: Matrix4<f32> = perspective(Deg(45.0), 800.0 / 600.0, 0.1, 100.0);
//             let proj_loc = uniform::uniform::uniform_location(program.program_id(), std::ffi::CString::new("projection").unwrap());
//             uniform::uniform::uniform_matrix_4fv(proj_loc, &projection);

//             let view = camera.get_view_matrix();
//             let view_loc = uniform::uniform::uniform_location(program.program_id(), std::ffi::CString::new("view").unwrap());
//             uniform::uniform::uniform_matrix_4fv(view_loc, &view);

//             let model = Matrix4::from_angle_x(Deg(0.0));
//             let model_loc = uniform::uniform::uniform_location(program.program_id(), std::ffi::CString::new("model").unwrap());
//             uniform::uniform::uniform_matrix_4fv(model_loc, &model);

//             vao.bind();
//             gl::DrawArrays(gl::TRIANGLES, 0, 36);
//         }

//     let mut should_quit = false;

// for (_, ev) in glfw::flush_messages(&window.event_handle) {
//     match ev {
//         glfw::WindowEvent::FramebufferSize(w, h) => unsafe {
//             gl::Viewport(0, 0, w, h);
//         },
//         // Modify the flag, NOT the window directly
//         glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
//             should_quit = true;
//         },
        
//         // ... (Keep your existing camera controls exactly the same)
//         glfw::WindowEvent::Key(glfw::Key::W, _, glfw::Action::Repeat | glfw::Action::Press, _) => {
//             camera.process_keyboard(CameraMovement::FORWARD, delta_time);
//         }
//         glfw::WindowEvent::Key(glfw::Key::S, _, glfw::Action::Repeat | glfw::Action::Press, _) => {
//             camera.process_keyboard(CameraMovement::BACKWARD, delta_time);
//         }
//         glfw::WindowEvent::Key(glfw::Key::A, _, glfw::Action::Repeat | glfw::Action::Press, _) => {
//             camera.process_keyboard(CameraMovement::LEFT, delta_time);
//         }
//         glfw::WindowEvent::Key(glfw::Key::D, _, glfw::Action::Repeat | glfw::Action::Press, _) => {
//             camera.process_keyboard(CameraMovement::RIGHT, delta_time);
//         }
//         glfw::WindowEvent::Key(glfw::Key::Left, _, glfw::Action::Repeat | glfw::Action::Press, _) => {
//             camera.process_mouse_movement(-50.0, 0.0, true);
//         }
//         glfw::WindowEvent::Key(glfw::Key::Right, _, glfw::Action::Repeat | glfw::Action::Press, _) => {
//             camera.process_mouse_movement(50.0, 0.0, true);
//         }
//         glfw::WindowEvent::Key(glfw::Key::Up, _, glfw::Action::Repeat | glfw::Action::Press, _) => {
//             camera.process_mouse_movement(0.0, 50.0, true);
//         }
//         glfw::WindowEvent::Key(glfw::Key::Down, _, glfw::Action::Repeat | glfw::Action::Press, _) => {
//             camera.process_mouse_movement(0.0, -50.0, true);
//         }
//         _ => {}
//     }
// }
//         if should_quit {
//            window.set_should_close(true);
// }

//         window.update();
//     }
// }

// fn load_shaders(vs_path: &str, fs_path: &str) -> ShaderProgram {
//     let vs_src = fs::file::load_shader(vs_path).unwrap();
//     let vs_c = fs::file::shader_to_cstring(vs_src).unwrap();
//     let vs = ShaderHandle::new_and_compile(&vs_c, backend::opengl::shader::ShaderType::Vertex);
//     vs.check_compile_status();

//     let fs_src = fs::file::load_shader(fs_path).unwrap();
//     let fs_c = fs::file::shader_to_cstring(fs_src).unwrap();
//     let fs = ShaderHandle::new_and_compile(&fs_c, backend::opengl::shader::ShaderType::Fragment);
//     fs.check_compile_status();

//     let prog = ShaderProgram::new();
//     prog.attach_shader(vs.shader);
//     prog.attach_shader(fs.shader);
//     prog.link_program();
//     prog.check_link_status();
    
//     vs.delete_shader();
//     fs.delete_shader();
//     prog
// }

// use std::f32::consts::PI;

// pub enum CameraMovement {
//     FORWARD,
//     BACKWARD,
//     LEFT,
//     RIGHT,
// }

// pub struct Camera {
//     pub position: Point3<f32>,
//     pub front: Vector3<f32>,
//     pub up: Vector3<f32>,
//     pub right: Vector3<f32>,
//     pub world_up: Vector3<f32>,
//     pub yaw: f32,
//     pub pitch: f32,
//     pub movement_speed: f32,
//     pub mouse_sensitivity: f32,
//     pub zoom: f32,
// }

// impl Camera {
//     pub fn new(position: Point3<f32>, up: Vector3<f32>, yaw: f32, pitch: f32) -> Camera {
//         let mut camera = Camera {
//             position,
//             front: vec3(0.0, 0.0, -1.0),
//             up,
//             right: vec3(0.0, 0.0, 0.0),
//             world_up: up,
//             yaw,
//             pitch,
//             movement_speed: 2.5,
//             mouse_sensitivity: 0.1,
//             zoom: 45.0,
//         };
//         camera.update_camera_vectors();
//         camera
//     }

//     pub fn get_view_matrix(&self) -> Matrix4<f32> {
//         Matrix4::look_at_rh(self.position, self.position + self.front, self.up)
//     }

//     pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
//         let velocity = self.movement_speed * delta_time;
//         match direction {
//             CameraMovement::FORWARD => self.position += self.front * velocity,
//             CameraMovement::BACKWARD => self.position -= self.front * velocity,
//             CameraMovement::LEFT => self.position -= self.right * velocity,
//             CameraMovement::RIGHT => self.position += self.right * velocity,
//         }
//     }

//     pub fn process_mouse_movement(&mut self, mut xoffset: f32, mut yoffset: f32, constrain_pitch: bool) {
//         xoffset *= self.mouse_sensitivity;
//         yoffset *= self.mouse_sensitivity;

//         self.yaw += xoffset;
//         self.pitch += yoffset;

//         if constrain_pitch {
//             if self.pitch > 89.0 { self.pitch = 89.0; }
//             if self.pitch < -89.0 { self.pitch = -89.0; }
//         }

//         self.update_camera_vectors();
//     }

//     fn update_camera_vectors(&mut self) {
//         let mut front = Vector3 {
//             x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
//             y: self.pitch.to_radians().sin(),
//             z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
//         };
//         front = front.normalize();
//         self.front = front;
//         self.right = front.cross(self.world_up).normalize();
//         self.up = self.right.cross(front).normalize();
//     }
// }
// fn vec3(x: f32, y: f32, z: f32) -> Vector3<f32> {
//     Vector3::new(x, y, z)
// }