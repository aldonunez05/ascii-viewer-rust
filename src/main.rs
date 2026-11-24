mod camera;
mod canvas;
mod model;
mod raster;
mod shading;
mod triangularization;

use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use glam::Vec3;

fn render(m: &model::Model, tris: &[[usize; 3]], cam: &camera::Camera, screen: &mut canvas::Canvas) {
    let (w, h) = (screen.width, screen.height);
    let mvp = cam.matrix(w, h);
    let rot = cam.rotation();
    let light = Vec3::new(-0.5, 0.8, 1.0).normalize();

    for &t in tris {
        let (va, vb, vc) = (m.vertices[t[0]], m.vertices[t[1]], m.vertices[t[2]]);
        let n = rot.transform_vector3(shading::normal(va, vb, vc));
        let ch = shading::shade(n, light);

        let a = camera::project(mvp, va, w, h);
        let b = camera::project(mvp, vb, w, h);
        let c = camera::project(mvp, vc, w, h);
        raster::draw_triangle(screen, a, b, c, ch);
    }
}

fn main() -> Result<(), String> {
    let path = std::env::args().nth(1).unwrap_or_else(|| "../models/cube.obj".to_string());

    let mut m = model::Model::load_obj(&path)?;
    m.normalize();
    let tris = m.triangles();

    let mut screen = canvas::Canvas::new(60, 25);
    let mut cam = camera::Camera::new();

    print!("\x1b[2J");
    loop {
        screen.clear();
        render(&m, &tris, &cam, &mut screen);
        print!("\x1b[H{screen}");
        std::io::stdout().flush().unwrap();

        cam.yaw += 0.04;
        sleep(Duration::from_millis(33));
    }
}
