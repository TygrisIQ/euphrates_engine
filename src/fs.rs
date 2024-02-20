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
    use image::{DynamicImage, GenericImageView};

    pub fn load_image(path: &str) -> image::DynamicImage {
        use std::path::Path;
        let path = Path::new(path);
        let img = image::open(&path).expect("FAILED TO OPEN IMAGE");

        return img;
    }
    pub fn image_pixels(img: &DynamicImage) -> Vec<u8> {
        let bid = img.clone().into_rgb8();
        let mut vect = Vec::new();
        for pixel in bid.pixels() {
            let rgb = pixel.0;
            vect.push(rgb[0]);
            vect.push(rgb[1]);
            vect.push(rgb[2]);
        }

        return vect;
    }
    fn write_vector(v: &Vec<u8>) {
        use std::fs::File;
        use std::io::{Result, Write};
        let mut file = File::create("img.txt").unwrap();
        for &byte in v {
            file.write_all(&[byte]).unwrap();
        }
    }
}
