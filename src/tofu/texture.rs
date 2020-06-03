extern crate stb_image;

use stb_image::*;

use gl::types::*;

use std::ffi::c_void;
use std::path::Path;
use std::ptr;

const TEXTURE_FORMAT: [GLenum; 4] = [gl::RED, gl::RG, gl::RGB, gl::RGBA];

#[derive(Clone, Default)]
pub struct Texture {
    pub id: GLuint,
    target: GLenum,
    internal_format: GLenum,
    format: GLenum,
    data_type: GLenum,
}

impl Texture {
    pub fn new(
        target: GLenum,
        internal_format: GLenum,
        format: GLenum,
        data_type: GLenum,
        width: u32,
        height: u32,
    ) -> Texture {
        let mut texture = Texture {
            target,
            internal_format,
            format,
            data_type,
            ..Texture::default()
        };

        unsafe {
            gl::GenTextures(1, &mut texture.id);
            gl::BindTexture(target, texture.id);

            gl::TexImage2D(
                target,
                0,
                texture.internal_format as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                texture.format,
                texture.data_type,
                ptr::null(),
            );

            gl::BindTexture(target, 0);
        }

        texture
    }

    pub fn load_from_file(image_filepath: &str) -> Texture {
        let mut texture = Texture {
            target: gl::TEXTURE_2D,
            ..Texture::default()
        };

        unsafe {
            gl::GenTextures(1, &mut texture.id);
            gl::BindTexture(texture.target, texture.id);

            if let image::LoadResult::ImageU8(texture_data) = image::load(Path::new(image_filepath))
            {
                texture.internal_format = TEXTURE_FORMAT[texture_data.depth - 1];
                texture.format = texture.internal_format;
                texture.data_type = gl::UNSIGNED_BYTE;

                gl::TexImage2D(
                    texture.target,
                    0,
                    texture.internal_format as GLint,
                    texture_data.width as GLsizei,
                    texture_data.height as GLsizei,
                    0,
                    texture.format,
                    texture.data_type,
                    texture_data.data.as_ptr() as *const u8 as *const c_void,
                );

                gl::GenerateMipmap(texture.target);
            } else {
                panic!("Failed to load image!");
            }

            texture.set_texture_wrapping(gl::REPEAT, gl::REPEAT);
            texture.set_min_mag_filters(gl::LINEAR_MIPMAP_LINEAR, gl::LINEAR);

            gl::BindTexture(texture.target, 0);
        }

        texture
    }

    pub fn bind_texture(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(slot);
            gl::BindTexture(self.target, self.id);
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        unsafe {
            gl::BindTexture(self.target, self.id);
            gl::TexImage2D(
                self.target,
                0,
                self.internal_format as GLint,
                new_width as GLsizei,
                new_height as GLsizei,
                0,
                self.format,
                self.data_type,
                ptr::null(),
            );
        }
    }

    pub fn set_texture_wrapping(&self, s_wrapping: GLenum, t_wrapping: GLenum) {
        unsafe {
            gl::BindTexture(self.target, self.id);

            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_S, s_wrapping as GLint);
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_T, t_wrapping as GLint);
        }
    }

    pub fn set_min_mag_filters(&self, min_filter: GLenum, mag_filter: GLenum) {
        unsafe {
            gl::BindTexture(self.target, self.id);

            gl::TexParameteri(self.target, gl::TEXTURE_MIN_FILTER, min_filter as GLint);
            gl::TexParameteri(self.target, gl::TEXTURE_MAG_FILTER, mag_filter as GLint);
        }
    }
}
