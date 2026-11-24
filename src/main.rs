mod model;
mod triangularization;

use glam::Vec3;

fn main() {
    let m = model::Model::load_obj("../models/cube.obj").unwrap();
    println!("{} vertices, {} faces, {} triangles", m.vertices.len(), m.faces.len(), m.triangles().len());
    m.normalize();
}
