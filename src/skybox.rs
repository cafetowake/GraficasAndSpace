use raylib::prelude::*;

pub struct Skybox {
    stars: Vec<Star>,
}

struct Star {
    direction: Vector3,
    brightness: f32,
    size: f32,
}

impl Skybox {
    pub fn new() -> Self {
        let mut stars = Vec::new();
        
        for i in 0..800 {
            let seed = i as f32;
            
            let theta = hash_1d(seed * 1.1) * std::f32::consts::TAU;
            let phi = (hash_1d(seed * 2.3) * 2.0 - 1.0).acos();
            
            let x = phi.sin() * theta.cos();
            let y = phi.sin() * theta.sin();
            let z = phi.cos();
            
            let direction = Vector3::new(x, y, z);
            
            let brightness = 0.3 + hash_1d(seed * 3.7) * 0.7;
            let size = if hash_1d(seed * 4.2) > 0.95 {
                0.08 
            } else if hash_1d(seed * 5.1) > 0.8 {
                0.05 
            } else {
                0.03 
            };
            
            stars.push(Star {
                direction,
                brightness,
                size,
            });
        }
        
        Self { stars }
    }
    
    pub fn render(&self, d: &mut RaylibMode3D<RaylibDrawHandle>, camera_pos: Vector3) {
        let distance = 100.0; 
        
        for star in &self.stars {
            let pos = Vector3::new(
                camera_pos.x + star.direction.x * distance,
                camera_pos.y + star.direction.y * distance,
                camera_pos.z + star.direction.z * distance,
            );
            
            let intensity = (star.brightness * 255.0) as u8;
            let color = Color::new(intensity, intensity, intensity, 255);
            
            d.draw_sphere(pos, star.size, color);
        }
    }
}

fn hash_1d(x: f32) -> f32 {
    let xi = (x * 12.9898).sin() * 43758.5453;
    xi - xi.floor()
}