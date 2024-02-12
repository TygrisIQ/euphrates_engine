use std::ffi::CStr;

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

    pub fn load_image_pixels<T>(path: &str) -> Vec<u8> {
        use std::path::Path;
        let path = Path::new(path);
        let img = image::open(&path).expect("FAILED TO OPEN IMAGE");
        //let pixel = img.into_rgba8().into_raw();
        //check if the image is of rgba8 format, which will be used in our textures, if not panic
        let buffer = match img {
            image::DynamicImage::ImageRgba8(buffer) => buffer.to_vec(),
            _ => panic!("unsupported image format!"),
        };
        buffer
    }
}
