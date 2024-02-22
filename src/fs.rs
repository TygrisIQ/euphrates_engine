pub mod fs {
    use std::{ffi::CString, io::Result, path::Path};

    pub fn shader_to_cstring(shader: String) -> Result<CString> {
        Ok(CString::new(shader.as_bytes()).unwrap())
    }
    pub fn load_shader(shader_name: &str) -> Result<String> {
        use std::fs;

        let path = Path::new(shader_name);
        return fs::read_to_string(path);
    }
}

pub mod image_handle {

    use image::{self, DynamicImage, GenericImage};

    //loads image as a Dynamic image enum
    pub fn load_image(path: &str) -> image::DynamicImage {
        use std::path::Path;
        let path = Path::new(path);
        let img = image::open(path).expect("FAILED TO OPEN IMAGE");

        img
    }
    pub fn image_pixels(img: &DynamicImage) -> Vec<u8> {
        let bid = img.to_rgb8();
        let data: Vec<u8> = bid
            .pixels()
            .flat_map(|pixel| vec![pixel[0], pixel[1], pixel[2]])
            .collect();

        return data;
    }
    fn write_data(v: &Vec<u8>) {
        use std::fs::File;
        use std::io::{Result, Write};

        print!("{}{}{}", v[0], v[1], v[2]);

        let mut file = File::create("target/img.txt").unwrap();

        for &byte in v {
            file.write_all(&[byte]).unwrap();
        }
    }
}
