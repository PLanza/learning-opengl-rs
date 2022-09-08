use std::{path::Path, ffi::CStr};

pub struct Shader{
    program_id: u32,
}

impl Shader {
    pub fn new(vertex_path: String, fragment_path: String) -> Result<Shader, String> {
        let vertex_code = std::fs::read_to_string(Path::new(&vertex_path)).map_err(|e| e.to_string())?;
        println!("{}", vertex_code);
        let fragment_code = std::fs::read_to_string(Path::new(&fragment_path)).map_err(|e| e.to_string())?;

        let v_shader_code =
            std::ffi::CString::new(vertex_code.as_bytes()).map_err(|e| e.to_string())?;
        let f_shader_code =
            std::ffi::CString::new(fragment_code.as_bytes()).map_err(|e| e.to_string())?;
       
        let program_id = unsafe {
            let (vertex_shader, fragment_shader): (u32, u32);

            // Create the vertex shader
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &v_shader_code.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(vertex_shader);

            // Check for GLSL compilation errors
            let mut success = gl::FALSE as gl::types::GLint;
            let mut info_log = Vec::with_capacity(512);

            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );
                let error = format!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    std::str::from_utf8(&info_log).unwrap()
                );
                return Err(error);
            }

            // Create the fragment shader
            fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &f_shader_code.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(fragment_shader);

            // Check for GLSL compilation errors
            let mut success = gl::FALSE as gl::types::GLint;
            let mut info_log = Vec::with_capacity(512);

            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );
                let error = format!(
                    "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                    std::str::from_utf8(&info_log).unwrap()
                );
                return Err(error);
            }

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);

            // Check for program link errors
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetProgramInfoLog(
                    program_id,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );
                println!(
                    "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                    std::str::from_utf8(&info_log).unwrap()
                );
            }

            // Delete Shaders
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            program_id
        };


        Ok(Shader {program_id})
    }
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.program_id);
    }

    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.program_id, name.as_ptr()), value as i32);    
    }

    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.program_id, name.as_ptr()), value);    
    }

    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.program_id, name.as_ptr()), value);    
    }
}
