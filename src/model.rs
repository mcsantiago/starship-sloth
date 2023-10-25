use crate::geometry::Vec3f;

pub struct Model {
    pub verts: Vec<Vec3f>,
    pub faces: Vec<Vec<i32>>
}

impl Model {
    pub fn new(filename: &str) -> Self {
        let mut verts: Vec<Vec3f> = Vec::new();
        let mut faces: Vec<Vec<i32>> = Vec::new();

        // Validate filename is obj
        if !filename.ends_with(".obj") {
            panic!("Invalid file type. Must be .obj");
        }

        // Read file
        let contents = std::fs::read_to_string(filename)
            .expect("Something went wrong reading the file");


        // for each line in contents, read the line as Vec3f
        for line in contents.split("\n") {
            if line.starts_with("#") || line.is_empty() {
                continue;
            }
            let values: Vec<&str> = line.split_whitespace().collect();
            if values[0].eq("v") {
                let x = values[1].parse::<f32>().unwrap();
                let y = values[2].parse::<f32>().unwrap();
                let z = values[3].parse::<f32>().unwrap();
                let vert: Vec3f = Vec3f { x, y, z };
                verts.push(vert);
            }
            else if values[0].eq("f") {
                // f 1/1/1 2/2/2 3/3/3
                let mut face: Vec<i32> = Vec::new();
                for i in 1..values.len() {
                    let face_values: Vec<&str> = values[i].split("/").collect();
                    let vert_index = face_values[0].parse::<i32>().unwrap();
                    let tex_index = face_values[1].parse::<i32>().unwrap();
                    let norm_index = face_values[2].parse::<i32>().unwrap();
                    face.push(vert_index - 1);
                }
                faces.push(face);
            }
        }

        println!("Model loaded: {} verts, {} faces", verts.len(), faces.len());

        Self { verts, faces }
    }
}
