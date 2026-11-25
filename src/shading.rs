use glam::Vec3;

const RAMP: [char; 9] = ['.', ':', '-', '=', '+', '*', '#', '%', '@'];

pub fn normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    (b - a).cross(c - a).normalize_or_zero()
}

pub fn shade(normal: Vec3, light: Vec3) -> char {
    let brightness = normal.dot(light).max(0.0);
    let i = (brightness * (RAMP.len() - 1) as f32).round() as usize;
    RAMP[i]
}
