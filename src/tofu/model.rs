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

            let mut vertices: Vec<tofu::mesh::Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();

            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(tofu::mesh::Vertex {
                    position: vec3(p[i * 3], p[i * 3 + 1], p[i * 3 + 2]),
                    normal: vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2]),
                    uv: vec2(t[i * 2], 1.0 - t[i * 2 + 1]),
                });
            }

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
