extern crate image;

use gl::types::*;

use std::ffi::c_void;
use std::path::Path;

pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new(image_filepath: &str) -> Texture {
        let mut texture = Texture { id: 0 };

        unsafe {
            gl::GenTextures(1, &mut texture.id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as GLint,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            let texture_data = image::open(&Path::new(image_filepath))
                .expect("Failed to load texture!")
                .flipv()
                .into_rgba();
            let (w, h) = (texture_data.width(), texture_data.height());
            let texture_blob = texture_data.into_raw();
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                w as GLsizei,
                h as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &texture_blob[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        texture
    }

    pub unsafe fn bind(&self, slot: GLenum) {
        gl::ActiveTexture(slot);
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
}
