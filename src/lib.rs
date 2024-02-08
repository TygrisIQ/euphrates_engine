pub use crate::backend::window::eupwindow::eup_window::EupWindow;

pub mod backend;
pub fn add(left: usize, right: usize) {
    use crate::backend::window::eupwindow;
    let window = EupWindow::new("title", 640, 480, (3, 3));
}
