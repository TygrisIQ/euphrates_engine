use euphrates_engine::{self};

fn main() {
    let mut window = euphrates_engine::EupWindow::new("window", 640, 480, (3, 3));

    while !window.should_close() {
        euphrates_engine::EupWindow::update(&mut window);
    }
}
