use euphrates_engine::fs::image_handle::load_image_pixels;

fn main() {
    use euphrates_engine::fs::image_handle;

    dbg!(load_image_pixels("texture/wall.jpg").unwrap());
    let tex_coords: [f32; 6] = [
        0.0, 0.0, //
        1.0, 0.0, //
        0.5, 1.0,
    ];
}
