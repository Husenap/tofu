use cgmath::*;
use cgmath::{Vector2, Vector3};

use std::ffi::c_void;
use std::mem;
use std::ptr;

use gl::types::*;

#[allow(dead_code)]
pub struct Vertex {
    position: Vector3<f32>,
    normal: Vector3<f32>,
    uv: Vector2<f32>,
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            position: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 1.0),
            uv: vec2(0.0, 0.0),
        }
    }
}

pub struct Model {
    vbo: GLuint,
    vao: GLuint,
    ebo: GLuint,
    num_indices: GLsizei,
}

impl Model {
    pub fn new(model_filepath: &str) -> Model {
        let mut importer = assimp::Importer::new();

        importer.triangulate(true);
        importer.join_identical_vertices(true);
        importer.optimize_meshes(true);
        importer.generate_normals(|n| {
            n.enable = true;
            n.smooth = true;
        });
        importer.calc_tangent_space(|t| t.enable = true);
        importer.flip_uvs(true);

        let scene = importer
            .read_file(model_filepath)
            .expect("Failed to load model!");

        let mut model = Model {
            vbo: 0,
            vao: 0,
            ebo: 0,
            num_indices: 0,
        };

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for mesh in scene.mesh_iter() {
            vertices.reserve_exact(mesh.num_vertices() as usize);

            model.num_indices = 3 * mesh.num_faces() as GLsizei;
            indices.reserve_exact(model.num_indices as usize);

            for index in 0..mesh.num_vertices() {
                let pos = mesh.get_vertex(index).unwrap();
                let nor = mesh.get_normal(index).unwrap();
                let uv = mesh.get_texture_coord(0, index).unwrap();
                vertices.push(Vertex {
                    position: vec3(pos.x, pos.y, pos.z),
                    normal: vec3(nor.x, nor.y, nor.z),
                    uv: vec2(uv.x, uv.y),
                });
            }

            for index in 0..mesh.num_faces() {
                let face = mesh.get_face(index).unwrap();
                indices.push(face[0]);
                indices.push(face[1]);
                indices.push(face[2]);
            }
        }

        unsafe {
            gl::GenVertexArrays(1, &mut model.vao);
            gl::GenBuffers(1, &mut model.vbo);
            gl::GenBuffers(1, &mut model.ebo);

            gl::BindVertexArray(model.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, model.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<Vertex>() * vertices.len()) as GLsizeiptr,
                &vertices[0] as *const _ as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, model.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mem::size_of::<u32>() * indices.len()) as GLsizeiptr,
                &indices[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of_val(&vertices[0]) as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of_val(&vertices[0]) as GLsizei,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of_val(&vertices[0]) as GLsizei,
                (6 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        model
    }

    pub unsafe fn use_model(&self) {
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
    }

    pub unsafe fn draw(&self) {
        gl::DrawElements(
            gl::TRIANGLES,
            self.num_indices as GLsizei,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
    }
}
