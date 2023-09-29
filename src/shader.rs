use std::ffi::CString;

use anyhow::{anyhow, Context, Ok, Result};
use gl::types::{GLchar, GLenum, GLint, GLuint};

use crate::pipeline::Expr;

#[derive(Debug)]
pub struct ShaderProgram {
    frag_shader: GLuint,
    vert_shader: GLuint,
    pub program_id: GLuint,
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

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn uniform_expr(&self, name: &str, v: &Expr) -> Result<()> {
        match v {
            Expr::Float(v) => self.uniform_1f(name, *v)?,
            Expr::Vec2(v) => self.uniform_2f(name, *v)?,
            Expr::Vec3(v) => self.uniform_3f(name, *v)?,
            Expr::Vec4(v) => self.uniform_4f(name, *v)?,
            Expr::String(v) => {
                if !v.starts_with('#') {
                    return Err(anyhow!("Only color string are supported as uniforms"));
                }
                let r = Self::str_to_f32(&v[1..3])?;
                let g = Self::str_to_f32(&v[3..5])?;
                let b = Self::str_to_f32(&v[5..7])?;
                self.uniform_3f(name, [r, g, b])?;
            }
        }
        Ok(())
    }

    fn str_to_f32(s: &str) -> Result<f32> {
        let mut res = 0;
        for c in s.chars() {
            res *= 16;
            res += Self::char_as_digit(c)?;
        }

        Ok(res as f32 / 255.0)
    }

    fn char_as_digit(c: char) -> Result<u32> {
        if c.is_ascii_digit() {
            Ok(c as u32 - '0' as u32)
        } else if c >= 'a' || c <= 'f' {
            Ok(c as u32 - 'a' as u32 + 10)
        } else {
            Err(anyhow!("Expect only digits in color string, got '{c}'"))
        }
    }

    pub fn uniform_1i(&self, name: &str, v: i32) -> Result<()> {
        self.uniform(name, |id| unsafe { gl::Uniform1i(id, v) })
    }

    pub fn uniform_1f(&self, name: &str, v: f32) -> Result<()> {
        self.uniform(name, |id| unsafe {
            if self.uniform_type(id) == gl::FLOAT {
                gl::Uniform1f(id, v)
            } else {
                gl::Uniform1i(id, v as i32)
            }
        })
    }

    pub fn uniform_2f(&self, name: &str, v: [f32; 2]) -> Result<()> {
        self.uniform(name, |id| unsafe {
            if self.uniform_type(id) == gl::FLOAT_VEC2 {
                gl::Uniform2f(id, v[0], v[1])
            } else {
                gl::Uniform2i(id, v[0] as i32, v[1] as i32)
            }
        })
    }

    pub fn uniform_3f(&self, name: &str, v: [f32; 3]) -> Result<()> {
        self.uniform(name, |id| unsafe {
            if self.uniform_type(id) == gl::FLOAT_VEC3 {
                gl::Uniform3f(id, v[0], v[1], v[2])
            } else {
                gl::Uniform3i(id, v[0] as i32, v[1] as i32, v[2] as i32)
            }
        })
    }

    pub fn uniform_4f(&self, name: &str, v: [f32; 4]) -> Result<()> {
        self.uniform(name, |id| unsafe {
            if self.uniform_type(id) == gl::FLOAT_VEC4 {
                gl::Uniform4f(id, v[0], v[1], v[2], v[3])
            } else {
                gl::Uniform4i(id, v[0] as i32, v[1] as i32, v[2] as i32, v[3] as i32)
            }
        })
    }

    fn uniform<F: Fn(i32)>(&self, name: &str, f: F) -> Result<()> {
        let uniform_id = self.get_uniform_location(name)?;

        unsafe {
            gl::UseProgram(self.program_id);
        }

        f(uniform_id);

        Ok(())
    }

    fn uniform_type(&self, id: i32) -> u32 {
        let mut length = 0;
        let mut size = 0;
        let mut typ = 0;
        let mut name = 0;
        unsafe {
            gl::GetActiveUniform(
                self.program_id,
                id as u32,
                0,
                &mut length,
                &mut size,
                &mut typ,
                &mut name,
            );
        }
        typ
    }

    pub fn get_uniform_location(&self, name: &str) -> Result<i32> {
        let c_name = std::ffi::CString::new(name).unwrap();
        let uniform_id = unsafe {
            gl::GetUniformLocation(self.program_id, c_name.as_ptr() as *const gl::types::GLchar)
        };

        if uniform_id == -1 {
            return Err(anyhow!("Could not find uniform {} in program", name));
        }

        Ok(uniform_id)
    }

    pub fn get_attrib_location(&self, name: &str) -> Result<i32> {
        let c_name = std::ffi::CString::new(name).unwrap();
        let uniform_id = unsafe {
            gl::GetAttribLocation(self.program_id, c_name.as_ptr() as *const gl::types::GLchar)
        };

        if uniform_id == -1 {
            return Err(anyhow!("Could not find attrib {} in program", name));
        }

        Ok(uniform_id)
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
