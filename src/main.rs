mod model;
mod camera;
mod raster;
mod canvas;
mod triangularization;

use glam::Vec3;

fn main() {
    let mut m = model::Model::load_obj("../models/cube.obj").unwrap();
    m.normalize();

    let (w, h) = (60, 25);
    let mut screen = canvas::Canvas::new(w, h);
    let cam = camera::Camera::new();
    let mvp = cam.matrix(w, h);

    let chars = ['#', '@', '%', '*', '+', '='];
    for (i, &t) in m.triangles().iter().enumerate() {
        let a = camera::project(mvp, m.vertices[t[0]], w, h);
        let b = camera::project(mvp, m.vertices[t[1]], w, h);
        let c = camera::project(mvp, m.vertices[t[2]], w, h);
        raster::draw_triangle(&mut screen, a, b, c, chars[i / 2 % 6]);
    }
    print!("{screen}");
}
