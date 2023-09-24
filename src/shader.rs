use std::ffi::CString;

use anyhow::{anyhow, Context, Ok, Result};
use gl::types::{GLchar, GLenum, GLint, GLuint};

pub struct ShaderProgram {
    frag_shader: GLuint,
    vert_shader: GLuint,
    program_id: GLuint,
}

impl ShaderProgram {
    pub fn new(vert: &str, frag: &str) -> Result<Self> {
        let frag_shader = Self::create_shader(frag, gl::FRAGMENT_SHADER)
            .with_context(|| "Failed to frag shader")?;
        let vert_shader = Self::create_shader(vert, gl::VERTEX_SHADER)
            .with_context(|| "Failed to vert shader")?;
        let program_id = Self::create_program(frag_shader, vert_shader)
            .with_context(|| "Failed to load program")?;

        let program = ShaderProgram {
            frag_shader,
            vert_shader,
            program_id,
        };
        Ok(program)
    }

    fn create_program(frag_shader: GLuint, vert_shader: GLuint) -> Result<GLuint> {
        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, frag_shader);
            gl::AttachShader(id, vert_shader);
            gl::LinkProgram(id);
        }
        Self::program_link_status(id)?;
        unsafe {
            gl::DetachShader(id, frag_shader);
            gl::DetachShader(id, vert_shader);
        }

        Ok(id)
    }

    fn create_shader(source: &str, typ: GLenum) -> Result<GLuint> {
        let source = CString::new(source).unwrap();

        let id = unsafe { gl::CreateShader(typ) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        Self::shader_compile_status(id)?;

        Ok(id)
    }

    fn shader_compile_status(id: GLuint) -> Result<()> {
        let mut success: GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success != 0 {
            return Ok(());
        }
        let mut len: GLint = 0;

        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring(len as usize);
        let error_ptr = error.as_ptr() as *mut GLchar;

        unsafe {
            gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error_ptr);
        }

        let error_str = error.to_string_lossy().into_owned();
        Err(anyhow!("Failed to compile shader: {error_str}"))
    }

    fn program_link_status(id: GLuint) -> Result<()> {
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success != 0 {
            return Ok(());
        }

        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring(len as usize);
        let error_ptr = error.as_ptr() as *mut GLchar;

        unsafe {
            gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), error_ptr);
        }

        let error_str = error.to_string_lossy().into_owned();
        Err(anyhow!("Failed to compile shader: {error_str}"))
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.frag_shader);
            gl::DeleteShader(self.vert_shader);
            gl::DeleteProgram(self.program_id);
        }
    }
}

fn create_whitespace_cstring(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
