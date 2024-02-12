pub use crate::backend::window::eupwindow::eup_window::EupWindow;
pub mod backend;
pub mod fs;

#[cfg(test)]
mod tests {
    extern crate gl;
    extern crate glfw;

    use crate::{
        backend::{opengl::shader::ShaderProgram, window::eupwindow},
        EupWindow,
    };
    fn load_gl() -> EupWindow {
        let mut window = eupwindow::eup_window::EupWindow::new("TEST", 20, 20, (3, 3));
        window.load_gl();
        window
    }

    #[test]
    fn program_id() {
        let mut window = load_gl();
        let program_id = ShaderProgram::new().program_id();
        window.update();
        assert_ne!(program_id, 0);
    }
}
