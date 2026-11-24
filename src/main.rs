mod model;
mod triangularization;

use glam::Vec3;

fn main() {
    let square = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];
    let tris = triangularization::triangularize(&square);
    println!("{:?}", tris);
}
