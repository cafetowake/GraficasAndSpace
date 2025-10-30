use raylib::prelude::*;
use crate::shaders::{color_rock, color_gas};
use crate::utils::vec3_len;

#[derive(Clone, Copy, PartialEq, Eq)]

pub enum PlanetKind { Rocky, Gaseous }

pub struct Planet {
    #[allow(dead_code)]
    pub name: String,
    pub kind: PlanetKind,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub radius: f32,
    pub base_color: Color,
    pub has_rings: bool,
    pub has_moon: bool,
    pub angle: f32,
    pub moon_angle: f32,
}

impl Planet {

    pub fn new(
        name: &str,
        kind: PlanetKind,
        orbit_radius: f32,
        orbit_speed: f32,
        radius: f32,
        base_color: Color,
        has_rings: bool,
        has_moon: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            kind,
            orbit_radius,
            orbit_speed,
            radius,
            base_color,
            has_rings,
            has_moon,
            angle: 0.0,
            moon_angle: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, enabled: bool, time: f32) {
        if enabled {
            self.angle += dt * self.orbit_speed;
            if self.has_moon {
                self.moon_angle += dt * (self.orbit_speed * 6.0);
            }
        } else {
            let _ = time;
        }
    }

    pub fn position(&self) -> Vector3 {
        Vector3::new(self.orbit_radius * self.angle.cos(), 0.0, self.orbit_radius * self.angle.sin())
    }

    pub fn moon_position(&self) -> Option<Vector3> {
        if self.has_moon {
            let moon_orbit = self.radius * 2.5;
            let base = self.position();
            Some(Vector3::new(
                base.x + moon_orbit * self.moon_angle.cos(),
                base.y + 0.0,
                base.z + moon_orbit * self.moon_angle.sin(),
            ))
        } else {
            None
        }
    }

    pub fn shaded_color(&self, time: f32, _sun_pos: Vector3, camera_pos: Vector3) -> Color {
        let pos = self.position();

        let mut normal = Vector3::new(camera_pos.x - pos.x, camera_pos.y - pos.y, camera_pos.z - pos.z);
        let nlen = vec3_len(normal);
        if nlen == 0.0 {
            normal = Vector3::new(0.0, 0.0, 1.0);
        } else {
            normal.x /= nlen; normal.y /= nlen; normal.z /= nlen;
        }

        match self.kind {
            PlanetKind::Rocky => color_rock(normal, normal, self.base_color, time),
            PlanetKind::Gaseous => color_gas(normal, normal, self.base_color, time),
        }
    }
}
