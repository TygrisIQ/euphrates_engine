extern crate glfw;

pub mod eup_window {
    use glfw::{Context, Glfw};
    /// window struct, glfw handle (public), other handlers (private)
    pub struct EupWindow {
        pub glfw_handle: Glfw,
        window_handle: glfw::PWindow,
        pub event_handle: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    }
    impl EupWindow {
        pub fn new(title: &str, w: u32, h: u32, context_version: (u32, u32)) -> EupWindow {
            let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
            glfw.window_hint(glfw::WindowHint::ContextVersion(
                context_version.0,
                context_version.1,
            ));
            glfw.window_hint(glfw::WindowHint::OpenGlProfile(
                glfw::OpenGlProfileHint::Core,
            ));
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
            let (mut window, events) = glfw
                .create_window(w, h, title, glfw::WindowMode::Windowed)
                .expect("");
            glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
            window.make_current();
            window.set_key_polling(true);
            window.set_framebuffer_size_polling(true);
            return EupWindow {
                glfw_handle: glfw,
                window_handle: window,
                event_handle: events,
            };
        }
        pub fn return_events(&mut self) -> &mut glfw::GlfwReceiver<(f64, glfw::WindowEvent)> {
            &mut self.event_handle
        }
        /// ## load opengl function pointers
        pub fn load_gl(&mut self) {
            gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
        }
        /// ## poll events, handle events, tell window to swap buffers
        pub fn update(&mut self) {
            self.glfw_handle.poll_events();
            self.handle_events();
            self.window_handle.swap_buffers();
        }

        pub fn should_close(&self) -> bool {
            return self.window_handle.should_close();
        }
        /// ## this method is called from the update method, it handles changes in the FramebufferSize and key presses
        fn handle_events(&mut self) {
            for (_, ev) in glfw::flush_messages(&self.event_handle) {
                match ev {
                    glfw::WindowEvent::FramebufferSize(w, h) => unsafe {
                        gl::Viewport(0, 0, w, h);
                    },
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                        self.window_handle.set_should_close(true);
                    }
                    glfw::WindowEvent::Key(glfw::Key::Y, _, glfw::Action::Press, _) => unsafe {
                        dbg!(gl::GetError());
                    },
                    glfw::WindowEvent::Key(glfw::Key::T, _, glfw::Action::Press, _) => {
                        dbg!(glfw::get_error());
                    }
                    glfw::WindowEvent::Key(glfw::Key::U, _, glfw::Action::Press, _) => unsafe {
                        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                    },
                    glfw::WindowEvent::Key(glfw::Key::I, _, glfw::Action::Press, _) => unsafe {
                        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                    },
                    glfw::WindowEvent::Key(glfw::Key::O, _, glfw::Action::Press, _) => unsafe {
                        gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT);
                    },

                    _ => {}
                }
            }
        }
    }
}
