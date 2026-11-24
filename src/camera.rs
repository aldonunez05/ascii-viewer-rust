use glam:: {Mat4, Vec3};

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub fov: f32,
}

impl Camera {
    pub fn new() -> Self{
        Self { yaw: 0.6, pitch: 0.4, distance: 4.0, fov: 60f32.to_radians() }
    }

    pub fn matrix(&self, width: usize, height: usize) -> Mat4 {
        let aspect = width as f32 / (height as f32 * 2.0);
        let proj = Mat4::perspective_rh(self.fov, aspect, 0.1, 100.0);
        let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, self.distance), Vec3::ZERO, Vec3::Y);
        let model = self.rotation(); 
        proj * view * model
    }

    pub fn rotation(&self) -> Mat4 {
        Mat4::from_rotation_x(self.pitch) * Mat4::from_rotation_y(self.yaw)
    }


}
pub fn project(mvp: Mat4, v: Vec3, width: usize, height: usize) -> Vec3 {
        let clip = mvp * v.extend(1.0);
        let ndc = clip.truncate() / clip.w;
        Vec3::new(
            (ndc.x + 1.0) * 0.5 * width as f32,
            (1.0 - ndc.y) * 0.5 * height as f32,
            clip.w

        )
    }

