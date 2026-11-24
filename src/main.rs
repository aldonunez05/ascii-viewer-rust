mod model;
mod canvas;
mod triangularization;

use glam::Vec3;

fn main() {
    let mut m = model::Model::load_obj("../models/cube.obj").unwrap();
    println!("{} vertices, {} faces, {} triangles", m.vertices.len(), m.faces.len(), m.triangles().len());
    m.normalize();
    let c = canvas::Canvas::new(40, 10);
    println!("{}x{}", c.width, c.height);
}
