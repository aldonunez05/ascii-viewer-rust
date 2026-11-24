use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Material{
    pub name: String,
    pub color: Vec3,
}

#[derive(Debug, Clone)]
pub struct Face {
    pub vertices: Vec<usize>,
    pub materials: Option<usize>,
}


#[derive(Debug, Clone)]
pub struct Model{
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub materials: Vec<Material>,
}


