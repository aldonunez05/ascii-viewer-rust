use glam::Vec3;
use crate::triangularization::triangularize;

#[derive(Debug, Clone)]
pub struct Material{
    pub name: String,
    pub color: Vec3,
}

#[derive(Debug, Clone)]
pub struct Face {
    pub vertices: Vec<usize>,
    pub material: Option<usize>,
}


#[derive(Debug, Default)]
pub struct Model{
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub materials: Vec<Material>,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_vertex(&mut self, v: Vec3) -> usize {
        self.vertices.push(v);
        self.vertices.len() - 1
    }
    
    pub fn add_face(&mut self, vertices: Vec<usize>, material: Option<usize>) -> Result<(), String>{
        self.faces.push(Face {vertices, material});
        Ok(())
    }

    pub fn triangles(&self) -> Vec<[usize; 3]> {
        let mut out = Vec::new();
        for face in &self.faces {
            let pts: Vec<Vec3> = face.vertices.iter().map(|&i| self.vertices[i]).collect();
            let local = triangularize(&pts);
            for t in local.chunks_exact(3) {
                out.push([face.vertices[t[0]], face.vertices[t[1]], face.vertices[t[2]]]);
            }
        }
        out
    }

    pub fn load_obj(path: &str) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let mut model = Model::new();

        for line in text.lines() {
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("v") => {
                    let mut nums = Vec::new();
                    for s in parts{
                        nums.push(s.parse::<f32>().map_err(|e| format!("bad num '{s}': {e}"))?);
                    }
                    if nums.len() < 3{
                        return Err(format!("verex needs 3 nums: {line}"));
                    }
                    model.add_vertex(Vec3::new(nums[0], nums[1], nums[2]));
                }
                Some("f") => {
                    let mut idxs = Vec::new();
                    for part in parts {
                        let first = part.split('/').next().unwrap();
                        let n: usize = first.parse().map_err(|e| format! ("bad face index"))?;
                        idxs.push(n - 1);
                    }
                    model.add_face(idxs, None)?;
                }
                _ => {}
            }
        }
        Ok(model)
    }
}


