use glam::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub zoom: f32,
    pub rotation_angle: f32,
    pub rotation_radius: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 5.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            aspect: width as f32 / height as f32,
            fov_y: 45.0,
            z_near: 0.1,
            z_far: 100.0,
            zoom: 1.0,
            rotation_angle: 0.0,
            rotation_radius: 5.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.position * self.zoom, self.target, self.up);
        let proj = Mat4::perspective_rh(
            self.fov_y.to_radians(),
            self.aspect,
            self.z_near,
            self.z_far,
        );
        proj * view
    }

    pub fn update_zoom(&mut self, delta: f32) {
        self.zoom = (self.zoom + delta * 0.1).clamp(0.1, 10.0);
    }

    pub fn update_position(&mut self) {
        self.position = Vec3::new(
            self.rotation_radius * self.rotation_angle.cos(),
            self.position.y,
            self.rotation_radius * self.rotation_angle.sin(),
        );
    }

    pub fn rotate(&mut self, delta: f32) {
        self.rotation_angle += delta * 0.02;
        self.update_position();
    }

    pub fn move_vertical(&mut self, delta: f32) {
        self.position.y += delta * 0.1;
    }
}