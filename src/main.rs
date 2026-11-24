mod model;
mod canvas;
mod triangularization;

use glam::Vec3;

fn main() {
    let mut m = model::Model::load_obj("../models/cube.obj").unwrap();
    println!("{} vertices, {} faces, {} triangles", m.vertices.len(), m.faces.len(), m.triangles().len());
    m.normalize();
    let mut c = canvas::Canvas::new(20, 5);
    for x in 0..20 {
        c.set(x, 2, 1.0, '-');
    }
    c.set(10, 0, 1.0, '*');
    c.set(10, 4, 1.0, '*');
    c.set(10, 2, 0.5, '+');   // closer than the '-', wins
    print!("{c}");
}
