struct Model{
    faces: Vec<Face>,
    vertexes: Vec<Vec3>,
    materials: Vec<Material>,
}

fn model_init() -> Box<Model>{
    Box::new(Model{
        faces: Vec::with_capacity(1),
        vertexes: Vec::with_capacity(1),
        materials: Vec::with_capacity(1),
    })
}
