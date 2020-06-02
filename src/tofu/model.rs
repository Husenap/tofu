use cgmath::*;

use std::path::Path;

use crate::tofu;

#[derive(Default)]
pub struct Model {
    pub meshes: Vec<tofu::mesh::Mesh>,
    pub textures_loaded: Vec<tofu::mesh::TextureData>,
    directory: String,
}

impl Model {
    pub fn new(model_filepath: &str) -> Model {
        let mut model = Model::default();
        model.load_model(model_filepath);
        model
    }

    pub fn draw(&self, shader: &tofu::Shader) {
        for mesh in &self.meshes {
            unsafe {
                mesh.draw(shader);
            }
        }
    }

    fn load_model(&mut self, model_filepath: &str) {
        let filepath = Path::new(model_filepath);

        self.directory = filepath
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_str()
            .unwrap()
            .into();

        let obj = tobj::load_obj(filepath, true);

        let (models, materials) = obj.unwrap();

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            let mut temp_binormals: Vec<Vector3<f32>> = Vec::with_capacity(num_vertices);
            let mut vertices: Vec<tofu::mesh::Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            // Load vertices
            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(tofu::mesh::Vertex {
                    position: vec3(p[i * 3], p[i * 3 + 1], p[i * 3 + 2]),
                    normal: vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2]),
                    uv: vec2(t[i * 2], 1.0 - t[i * 2 + 1]),
                    ..tofu::mesh::Vertex::default()
                });

                temp_binormals.push(Vector3::zero());
            }

            // Calculate tangent space
            for i in (0..indices.len()).step_by(3) {
                let (i0, i1, i2) = (
                    indices[i] as usize,
                    indices[i + 1] as usize,
                    indices[i + 2] as usize,
                );

                let (v0, v1, v2) = (&vertices[i0], &vertices[i1], &vertices[i2]);

                let (q1, q2) = (v1.position - v0.position, v2.position - v0.position);
                let (s1, s2, t1, t2) = (
                    v1.uv.x - v0.uv.x,
                    v2.uv.x - v0.uv.x,
                    (1.0 - v1.uv.y) - (1.0 - v0.uv.y),
                    (1.0 - v2.uv.y) - (1.0 - v0.uv.y),
                );

                let (tangent, binormal) = (
                    (t2 * q1 - t1 * q2).normalize(),
                    (-s2 * q1 + s1 * q2).normalize(),
                );

                vertices[i0].tangent += tangent;
                temp_binormals[i0] += binormal;
                vertices[i1].tangent += tangent;
                temp_binormals[i1] += binormal;
                vertices[i2].tangent += tangent;
                temp_binormals[i2] += binormal;
            }

            for i in 0..num_vertices {
                let v = &mut vertices[i];

                v.tangent = (v.tangent - v.normal * v.tangent.dot(v.normal)).normalize();

                v.binormal_headedness = if v.normal.cross(v.tangent).dot(temp_binormals[i]) < 0.0 {
                    -1.0
                } else {
                    1.0
                };
            }

            // Load textures
            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                if !material.diffuse_texture.is_empty() {
                    textures.push(
                        self.load_material_texture(&material.diffuse_texture, "uAlbedoTexture"),
                    );
                }
                if !material.normal_texture.is_empty() {
                    textures.push(
                        self.load_material_texture(&material.normal_texture, "uNormalTexture"),
                    );
                }
                if !material.shininess_texture.is_empty() {
                    textures.push(
                        self.load_material_texture(
                            &material.shininess_texture,
                            "uRoughnessTexture",
                        ),
                    );
                }
                if !material.ambient_texture.is_empty() {
                    textures.push(
                        self.load_material_texture(&material.ambient_texture, "uMetallicTexture"),
                    );
                }
            }

            self.meshes
                .push(tofu::mesh::Mesh::new(vertices, indices, textures));
        }
    }

    fn load_material_texture(
        &mut self,
        texture_filepath: &str,
        texture_type: &str,
    ) -> tofu::mesh::TextureData {
        {
            let texture_data = self
                .textures_loaded
                .iter()
                .find(|t| t.filepath == texture_filepath);
            if let Some(texture_data) = texture_data {
                return texture_data.clone();
            }
        }

        let filepath = format!("{}/{}", self.directory, texture_filepath);
        let texture = tofu::Texture::new(&filepath);

        let texture_data = tofu::mesh::TextureData {
            texture,
            texture_type: String::from(texture_type),
            filepath: texture_filepath.into(),
        };

        self.textures_loaded.push(texture_data.clone());

        texture_data
    }
}
