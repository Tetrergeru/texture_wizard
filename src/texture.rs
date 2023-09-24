use std::ffi::c_void;

use anyhow::{Context, Result};
use gl::types::GLint;
use image::RgbaImage;

pub struct Texture {
    image: RgbaImage,
    id: gl::types::GLuint,
}

impl Texture {
    pub fn from_file(fname: &str) -> Result<Self> {
        let mut image = image::open(fname)
            .with_context(|| format!("Failed to read image from '{}'", fname))?
            .into_rgba8();

        let (w, h) = (image.width() as GLint, image.height() as GLint);
        
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as GLint,
                w,
                h,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.as_mut_ptr() as *mut c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Self { image, id: 0 })
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn save_to_file(&self, fname: &str) -> Result<()> {
        self.image.save(fname)?;
        Ok(())
    }
}
