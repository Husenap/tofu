use cgmath::{Vector2, Vector3};

use std::ffi::c_void;
use std::mem;
use std::ptr;

use gl::types::*;

use crate::tofu;

#[repr(C)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub uv: Vector2<f32>,
}

#[derive(Clone)]
pub struct TextureData {
    pub texture: tofu::Texture,
    pub texture_type: String,
    pub filepath: String,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<TextureData>,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<TextureData>) -> Mesh {
        let mut mesh = Mesh {
            vertices,
            indices,
            textures,
            vao: 0,
            vbo: 0,
            ebo: 0,
        };

        unsafe {
            mesh.setup_mesh();
        }

        mesh
    }

    pub unsafe fn draw(&self, shader: &tofu::Shader) {
        for (i, texture_data) in self.textures.iter().enumerate() {
            shader.set_int(&texture_data.texture_type, i as i32);
            texture_data.texture.bind(gl::TEXTURE0 + i as u32);
        }

        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as GLsizei,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
        gl::BindVertexArray(0);

        gl::ActiveTexture(gl::TEXTURE0);
    }

    unsafe fn setup_mesh(&mut self) {
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);
        gl::GenBuffers(1, &mut self.ebo);

        gl::BindVertexArray(self.vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        let size = (self.vertices.len() * mem::size_of::<Vertex>()) as GLsizeiptr;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        let size = (self.indices.len() * mem::size_of::<u32>()) as GLsizeiptr;
        let data = &self.indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        // Vertex Position
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<Vertex>() as GLsizei,
            ptr::null(),
        );

        // Vertex Normal
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<Vertex>() as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );

        // Vertex UV
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of::<Vertex>() as GLsizei,
            (6 * mem::size_of::<GLfloat>()) as *const c_void,
        );

        gl::BindVertexArray(0);
    }
}
