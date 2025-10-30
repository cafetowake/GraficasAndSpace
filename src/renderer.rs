use raylib::prelude::*;
use crate::planet::{Planet};

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self { Self }

    fn draw_orbit_circle(d: &mut RaylibMode3D<RaylibDrawHandle>, center: Vector3, r: f32) {
        let segments = 128;
        let mut prev = Vector3::new(center.x + r, center.y, center.z);
        for i in 1..=segments {
            let theta = (i as f32) * (std::f32::consts::TAU / segments as f32);
            let cur = Vector3::new(center.x + r * theta.cos(), center.y, center.z + r * theta.sin());
            d.draw_line_3D(prev, cur, Color::new(60, 60, 60, 100));
            prev = cur;
        }
    }

    fn draw_planet_ring(d: &mut RaylibMode3D<RaylibDrawHandle>, center: Vector3, inner: f32, outer: f32) {
        let segments = 64;
        for s in 0..segments {
            let a0 = (s as f32) * (std::f32::consts::TAU / segments as f32);
            let a1 = ((s + 1) as f32) * (std::f32::consts::TAU / segments as f32);
            let p0 = Vector3::new(center.x + inner * a0.cos(), center.y, center.z + inner * a0.sin());
            let p1 = Vector3::new(center.x + outer * a0.cos(), center.y, center.z + outer * a0.sin());
            let p2 = Vector3::new(center.x + inner * a1.cos(), center.y, center.z + inner * a1.sin());
            let p3 = Vector3::new(center.x + outer * a1.cos(), center.y, center.z + outer * a1.sin());
            d.draw_line_3D(p0, p1, Color::new(200, 190, 150, 220));
            d.draw_line_3D(p2, p3, Color::new(200, 190, 150, 220));
        }
    }

    pub fn render_scene(&mut self, d: &mut RaylibMode3D<RaylibDrawHandle>, planets: &[Planet], time: f32, show_grid: bool, camera_pos: Vector3) {
        let sun_pos = Vector3::zero();
        if show_grid {
            for i in -20..=20 {
                let start = Vector3::new(i as f32, 0.0, -20.0);
                let end   = Vector3::new(i as f32, 0.0,  20.0);
                d.draw_line_3D(start, end, Color::new(30,30,30,120));
                let start2 = Vector3::new(-20.0, 0.0, i as f32);
                let end2   = Vector3::new(20.0,  0.0, i as f32);
                d.draw_line_3D(start2, end2, Color::new(30,30,30,120));
            }
        }

        let sun_col = Color::new(255, 230, 110, 255);
        d.draw_sphere(sun_pos, 2.8, sun_col);
        let halo_col = Color::new(255, 240, 200, 60);
        d.draw_sphere(sun_pos, 3.8, halo_col);

        for p in planets {
            Self::draw_orbit_circle(d, Vector3::zero(), p.orbit_radius);
        }

        for p in planets.iter() {
            let pos = p.position();

            let tint = p.shaded_color(time, sun_pos, camera_pos);

            d.draw_sphere(pos, p.radius, tint);

            if p.has_rings {
                let inner = p.radius * 1.5;
                let outer = p.radius * 2.6; 
                Self::draw_planet_ring(d, pos, inner, outer);
            }

            if let Some(mpos) = p.moon_position() {
                d.draw_sphere(mpos, p.radius * 0.25, Color::new(150, 150, 150, 255));
            }
        }
    }
}
