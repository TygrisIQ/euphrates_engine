use std::ffi::CStr;

pub mod fs {
    use std::{ffi::CString, io::Result};

    pub fn shader_to_cstring(shader: String) -> Result<CString> {
        Ok(CString::new(shader.as_bytes()).unwrap())
    }
    pub fn load_shader(shader_name: &str) -> Result<String> {
        use std::{fs, path::Path};

        let path = Path::new(shader_name);
        return fs::read_to_string(path);
    }
}
