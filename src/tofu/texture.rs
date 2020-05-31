extern crate stb_image;

use stb_image::*;

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

            if let image::LoadResult::ImageU8(texture_data) = image::load(Path::new(image_filepath))
            {
                let source_format = if texture_data.depth == 3 {
                    gl::RGB
                } else {
                    gl::RGBA
                };

                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as GLint,
                    texture_data.width as GLsizei,
                    texture_data.height as GLsizei,
                    0,
                    source_format,
                    gl::UNSIGNED_BYTE,
                    texture_data.data.as_ptr() as *const u8 as *const c_void,
                );
                gl::GenerateMipmap(gl::TEXTURE_2D);
            } else {
                panic!("Failed to load image!");
            }
        }

        texture
    }

    pub unsafe fn bind(&self, slot: GLenum) {
        gl::ActiveTexture(slot);
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
}
