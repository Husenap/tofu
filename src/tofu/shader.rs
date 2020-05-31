use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use cgmath::prelude::*;
use cgmath::Matrix4;

use gl::types::*;

const LOG_SIZE: usize = 1024;

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(vertex_filepath: &str, fragment_filepath: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        let mut vertex_shader_file = File::open(vertex_filepath)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertex_filepath));
        let mut fragment_shader_file = File::open(fragment_filepath)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_filepath));

        let mut vertex_shader_code = String::new();
        let mut fragment_shader_code = String::new();

        vertex_shader_file
            .read_to_string(&mut vertex_shader_code)
            .expect("Failed to read vertex shader");
        fragment_shader_file
            .read_to_string(&mut fragment_shader_code)
            .expect("Failed to read fragment shader");

        let vertex_shader_blob = CString::new(vertex_shader_code.as_bytes()).unwrap();
        let fragment_shader_blob = CString::new(fragment_shader_code.as_bytes()).unwrap();

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &vertex_shader_blob.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            shader.check_compile_errors(vertex_shader, "VERTEX");

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &fragment_shader_blob.as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(fragment_shader);
            shader.check_compile_errors(fragment_shader, "FRAGMENT");

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            shader.check_link_errors(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            shader.id = program;
        }

        shader
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_float(&self, name: &str, value: f32) {
        let location = self.get_location(name);
        gl::Uniform1f(location, value);
    }

    pub unsafe fn set_mat4(&self, name: &str, value: &Matrix4<f32>) {
        gl::UniformMatrix4fv(self.get_location(name), 1, gl::FALSE, value.as_ptr());
    }

    pub unsafe fn set_int(&self, name: &str, value: i32) {
        gl::Uniform1i(self.get_location(name), value);
    }

    unsafe fn get_location(&self, name: &str) -> GLint {
        let safe_name = CString::new(name.as_bytes()).unwrap();
        gl::GetUniformLocation(self.id, safe_name.as_ptr() as *const GLchar)
    }

    unsafe fn check_compile_errors(&self, shader: u32, shader_type: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(LOG_SIZE);
        info_log.set_len(LOG_SIZE - 1);

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                shader,
                LOG_SIZE as GLsizei,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );

            println!(
                "ERROR::SHADER_COMPILATION_ERROR::{}\n{}\n",
                shader_type,
                str::from_utf8(&info_log).unwrap()
            );
        }
    }

    unsafe fn check_link_errors(&self, program: u32) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(LOG_SIZE);
        info_log.set_len(LOG_SIZE - 1);

        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
                program,
                LOG_SIZE as GLsizei,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );

            println!(
                "ERROR::PROGRAM_LINKER_ERROR\n{}\n",
                str::from_utf8(&info_log).unwrap()
            );
        }
    }
}
