use std::collections::HashMap;

use log::info;

use glam::Vec3;

#[derive(Debug)]
pub struct Model {
    pub verts: Vec<Vec3>,
    pub tex_coords: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub faces: Vec<Vec<(i32,i32,i32)>>,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct ModelId(usize);

pub struct ModelManager {
    pub models: HashMap<ModelId, Model>,
}

impl ModelManager {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    pub fn load_model(&mut self, model_filename: &str) -> ModelId {
        let model = Model::new(model_filename);
        self.add_model(model)
    }

    pub fn add_model(&mut self, model: Model) -> ModelId {
        let id = ModelId(self.models.len());
        self.models.insert(id, model);
        id
    }

    pub fn get_model(&self, id: ModelId) -> &Model {
        self.models.get(&id).unwrap()
    }
}

impl Model {
    pub fn new(model_filename: &str) -> Self {
        let mut verts: Vec<Vec3> = Vec::new();
        let mut tex_coords: Vec<Vec3> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut faces: Vec<Vec<(i32,i32,i32)>> = Vec::new();

        // Validate model_filename is obj
        if !model_filename.ends_with(".obj") {
            panic!("Invalid file type. Must be .obj");
        }

        // Read file
        let contents = std::fs::read_to_string(model_filename)
            .expect("Something went wrong reading the file");


        // for each line in contents, read the line as Vec3
        for line in contents.split("\n") {
            if line.starts_with("#") || line.is_empty() {
                continue;
            }
            let values: Vec<&str> = line.split_whitespace().collect();
            if values[0].eq("v") {
                let x = values[1].parse::<f32>().unwrap();
                let y = values[2].parse::<f32>().unwrap();
                let z = values[3].parse::<f32>().unwrap();
                let vert: Vec3 = Vec3 { x, y, z };
                verts.push(vert);
            }
            else if values[0].eq("vt") {
                let x = values[1].parse::<f32>().unwrap();
                let y = values[2].parse::<f32>().unwrap();
                let z = values[3].parse::<f32>().unwrap();
                let tex: Vec3 = Vec3 { x, y, z };
                tex_coords.push(tex);
            } 
            else if values[0].eq("vn") {
                let x = values[1].parse::<f32>().unwrap();
                let y = values[2].parse::<f32>().unwrap();
                let z = values[3].parse::<f32>().unwrap();
                let norm: Vec3 = Vec3 { x, y, z };
                normals.push(norm);
            }
            else if values[0].eq("f") {
                // f 1/1/1 2/2/2 3/3/3
                let mut face: Vec<(i32,i32,i32)> = Vec::new();
                for i in 1..values.len() {
                    let face_values: Vec<&str> = values[i].split("/").collect();
                    let vert_index = face_values[0].parse::<i32>().unwrap();
                    let tex_index = face_values[1].parse::<i32>().unwrap();
                    let norm_index = face_values[2].parse::<i32>().unwrap();
                    face.push((vert_index - 1, tex_index - 1, norm_index - 1));
                }
                faces.push(face);
            }
        }

        info!("Model loaded: {} verts, {} faces", verts.len(), faces.len());

        Self { verts, tex_coords, normals, faces }
    }
}
