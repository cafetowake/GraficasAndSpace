pub struct OrbitSystem {
    pub time: f32,
}

impl OrbitSystem {
    pub fn new() -> Self { Self { time: 0.0 } }
    pub fn advance(&mut self, dt: f32) { self.time += dt; }
}
