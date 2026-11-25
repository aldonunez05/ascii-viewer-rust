use crate::canvas::Canvas;
use glam::Vec3;

fn edge(a: Vec3, b: Vec3, p: Vec3) -> f32 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

pub fn draw_triangle(canvas: &mut Canvas, a: Vec3, b: Vec3, c: Vec3, ch: char) {
    let area = edge(a, b, c);
    if area == 0.0 {
        return;
    }
    let min_x = a.x.min(b.x).min(c.x).floor().max(0.0) as i32;
    let max_x = a.x.max(b.x).max(c.x).ceil().min(canvas.width as f32 - 1.0) as i32;
    let min_y = a.y.min(b.y).min(c.y).floor().max(0.0) as i32;
    let max_y = a.y.max(b.y).max(c.y).ceil().min(canvas.height as f32 - 1.0) as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            let w0 = edge(b, c, p) / area;
            let w1 = edge(c, a, p) / area;
            let w2 = edge(a, b, p) / area;
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let z = w0 * a.z + w1 * b.z + w2 * c.z;
                canvas.set(x, y, z, ch);
            }
        }
    }
}
