pub mod file {
    use std::{ffi::CString, io::Result, path::Path};
    /// # convert string to CString
    pub fn shader_to_cstring(shader: String) -> Result<CString> {
        Ok(CString::new(shader.as_bytes()).unwrap())
    }
    /// load shader code from file
    pub fn load_shader(shader_name: &str) -> Result<String> {
        use std::fs;

        let path = Path::new(shader_name);
        return fs::read_to_string(path);
    }
}

pub mod image_handle {

    use image::{self, DynamicImage};

    ///loads image as a Dynamic image enum,
    ///see <https://docs.rs/image/0.24.9/image/enum.DynamicImage.html>
    pub fn load_image(path: &str) -> image::DynamicImage {
        use std::path::Path;
        let path = Path::new(path);
        let img = image::open(path).expect("FAILED TO OPEN IMAGE").flipv();

        img
    }
    /// # retrun a `Vec<u8>` of *RGBA* values from an image
    pub fn image_pixels_rgba(img: &DynamicImage) -> Vec<u8> {
        let img = img.to_rgba8();
        let data: Vec<u8> = img
            .pixels()
            .flat_map(|pixel| vec![pixel[0], pixel[1], pixel[2], pixel[3]])
            .collect();
        return data;
    }
    /// # return a `Vec<u8>` of *RGB* values from an image
    pub fn image_pixels_rgb(img: &DynamicImage) -> Vec<u8> {
        let bid = img.to_rgb8();
        let data: Vec<u8> = bid
            .pixels()
            .flat_map(|pixel| vec![pixel[0], pixel[1], pixel[2]])
            .collect();

        return data;
    }
}
