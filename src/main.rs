mod model;
mod raster;
mod canvas;
mod triangularization;

use glam::Vec3;

fn main() {
    let mut m = model::Model::load_obj("../models/cube.obj").unwrap();
    println!("{} vertices, {} faces, {} triangles", m.vertices.len(), m.faces.len(), m.triangles().len());
    m.normalize();
    let mut c = canvas::Canvas::new(40, 15);
    raster::draw_triangle(&mut c,
        Vec3::new(2.0, 1.0, 5.0), Vec3::new(30.0, 3.0, 5.0), Vec3::new(8.0, 13.0, 5.0), '#');
    raster::draw_triangle(&mut c,
        Vec3::new(15.0, 2.0, 1.0), Vec3::new(38.0, 8.0, 1.0), Vec3::new(12.0, 14.0, 1.0), '.');
    print!("{c}");
}
