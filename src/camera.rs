use raylib::prelude::*;

pub struct CameraController {
    pub camera: Camera3D,
    yaw: f32,
    pitch: f32,
    distance: f32,
}

impl CameraController {
    pub fn new() -> Self {
        let camera = Camera3D::perspective(
            Vector3::new(12.0, 8.0, 20.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            45.0,
        );
        Self { camera, yaw: -0.8, pitch: -0.25, distance: 22.0 }
    }

    pub fn handle_input(&mut self, rl: &mut RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_A) { self.yaw -= 0.02; }
        if rl.is_key_down(KeyboardKey::KEY_D) { self.yaw += 0.02; }
        if rl.is_key_down(KeyboardKey::KEY_W) { self.distance -= 0.4; }
        if rl.is_key_down(KeyboardKey::KEY_S) { self.distance += 0.4; }

        let delta = rl.get_mouse_delta();
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
            self.yaw += delta.x * 0.01;
            self.pitch += delta.y * 0.01;
            self.pitch = self.pitch.clamp(-1.4, 1.4);
        }

        if self.distance < 4.0 { self.distance = 4.0; }

        let x = self.distance * self.yaw.cos() * self.pitch.cos();
        let z = self.distance * self.yaw.sin() * self.pitch.cos();
        let y = self.distance * (-self.pitch).sin();

        self.camera.position = Vector3::new(x, y + 2.0, z);
        self.camera.target = Vector3::zero();
    }
}
