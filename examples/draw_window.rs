use euphrates_engine::{self};

fn main() {
    let mut window = euphrates_engine::EupWindow::new("window", 640, 480, (3, 3));

    window.load_gl();
    while !window.should_close() {
        unsafe {
            window.update();
            extern crate gl;
            gl::ClearColor(1.0, 0.65, 0.0, 1.0);

            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
