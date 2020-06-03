use gl::types::*;

use crate::tofu;

#[derive(Default)]
pub struct Framebuffer {
    fbo: GLuint,
    render_targets: Vec<tofu::Texture>,
    depth_stencil: tofu::Texture,
}

#[derive(Clone)]
pub struct RenderTargetDescription {
    pub internal_format: GLenum,
    pub format: GLenum,
    pub data_type: GLenum,
}

impl Framebuffer {
    pub fn new(
        width: u32,
        height: u32,
        render_target_descriptions: Vec<RenderTargetDescription>,
    ) -> Framebuffer {
        let mut framebuffer = Framebuffer::default();
        framebuffer.setup(width, height, render_target_descriptions);
        framebuffer
    }

    fn setup(
        &mut self,
        width: u32,
        height: u32,
        render_target_descriptions: Vec<RenderTargetDescription>,
    ) {
        let mut attachments: Vec<GLenum> = Vec::with_capacity(render_target_descriptions.len());

        unsafe {
            gl::GenFramebuffers(1, &mut self.fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);

            for (i, description) in render_target_descriptions.iter().enumerate() {
                let render_target = tofu::Texture::new(
                    gl::TEXTURE_2D,
                    description.internal_format,
                    description.format,
                    description.data_type,
                    width,
                    height,
                );
                render_target.set_min_mag_filters(gl::NEAREST, gl::NEAREST);

                let attachment = ((gl::COLOR_ATTACHMENT0 as usize) + i) as GLenum;

                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    attachment,
                    gl::TEXTURE_2D,
                    render_target.id,
                    0,
                );

                self.render_targets.push(render_target);

                attachments.push(attachment);
            }

            self.depth_stencil = tofu::Texture::new(
                gl::TEXTURE_2D,
                gl::DEPTH24_STENCIL8,
                gl::DEPTH_STENCIL,
                gl::UNSIGNED_INT_24_8,
                width,
                height,
            );
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::TEXTURE_2D,
                self.depth_stencil.id,
                0,
            );

            gl::DrawBuffers(
                attachments.len() as GLsizei,
                &attachments[0] as *const GLenum,
            );

            let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            if status != gl::FRAMEBUFFER_COMPLETE {
                panic!(format!(
                    "{}: {}",
                    "Failed to initialize Framebuffer!", status
                ));
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.depth_stencil.resize(new_width, new_height);
        for render_target in self.render_targets.iter_mut() {
            render_target.resize(new_width, new_height);
        }
    }

    pub unsafe fn bind_as_target(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
    }

    pub unsafe fn unbind_as_target(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    pub unsafe fn bind_as_source(&self) {
        for (i, render_target) in self.render_targets.iter().enumerate() {
            render_target.bind_texture(gl::TEXTURE0 + i as u32);
        }
    }
}
