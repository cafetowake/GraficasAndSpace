use raylib::prelude::*;
use glam::{Vec3, Vec4};
use crate::engine::camera::Camera;

#[derive(Clone)]
pub struct Body {
    pub radius: f32,
    pub distance: f32,
    pub rotation_speed: f32,
    pub spin_speed: f32,
    pub spin_phase: f32,
    pub color_layers: [Vec4; 4],
    pub layer_thresholds: [f32; 3],
    pub has_ring: bool,
    pub has_moon: bool,
    pub is_star: bool,
    pub orbit_phase: f32,
}

pub struct RaylibRenderer {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub camera_rotation_x: f32,
    pub camera_rotation_y: f32,
    pub camera_scale: f32,
    pub camera_center: Vector2,
    pub zoom_speed: f32,
    pub camera: Camera,
    pub shader_layers: usize,
    pub system_rotation: f32,
    pub system_auto_rotation: bool,
    pub system_rotation_speed: f32,
    pub global_spin_enabled: bool,
    pub global_spin_speed: f32,
    pub show_orbits: bool,
    pub bodies: Vec<Body>,
}

impl RaylibRenderer {
    pub fn new(title: &str, width: i32, height: i32) -> Self {
        let (rl, thread) = raylib::init().size(width, height).title(title).build();
        Self {
            rl,
            thread,
            camera_rotation_x: 0.0,
            camera_rotation_y: 0.0,
            camera_scale: 5.0,
            camera_center: Vector2::new(400.0, 300.0),
            zoom_speed: 0.1,
            camera: Camera::new(width as u32, height as u32),
            shader_layers: 4,
            system_rotation: 0.0,
            system_auto_rotation: true,
            system_rotation_speed: 0.003,
            global_spin_enabled: true,
            global_spin_speed: 1.0,
            show_orbits: true,
            bodies: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

}

fn paint_sphere(
    d: &mut RaylibDrawHandle,
    center: Vector2,
    radius: f32,
    layers: &[Vec4;4],
    _thresholds: &[f32;3],
    _rot_x: f32,
    rot_y: f32,
    active_layers: usize,
    is_star: bool,
    light_dir: Option<Vec3>,
) {
    let steps = 60;  
    
    let light = if is_star {
        Vec3::new(0.0, 0.0, 1.0)
    } else if let Some(ld) = light_dir {
        ld
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    
    for i in (0..steps).rev() {
        let t = i as f32 / steps as f32;
        
        let phi = std::f32::consts::PI * (1.0 - t);
        let normal_y = phi.cos();
        let normal_r = phi.sin();
        
        let normal_x = normal_r * rot_y.cos();
        let normal_z = normal_r * rot_y.sin();
        
    let ndotl = (normal_x * light.x + normal_y * light.y + normal_z * light.z).max(0.0);
    let ambient = 0.02;
    let diffuse = ndotl.max(0.0);
    let light_dot = if is_star { 1.0 } else { (diffuse + ambient).min(1.0) };

    let mut specular = 0.0;
    if !is_star && ndotl > 0.0 {
        let view = Vec3::new(0.0, 0.0, 1.0);
        let half = (light + view).normalize();
        let ndoth = (normal_x * half.x + normal_y * half.y + normal_z * half.z).max(0.0);
        let shininess = 16.0;
        specular = ndoth.powf(shininess) * 0.6; 
    }
        let al = if active_layers == 0 { 1 } else { active_layers };
        let idx = ((t * al as f32) as usize).min(al.saturating_sub(1));
        let denom = (al as f32 - 1.0).max(1.0);
        let color_index = ((idx as f32 / denom) * 3.0).round() as usize;
        let mut color = layers[color_index.min(3)];
        color *= light_dot;
        color += Vec4::new(specular, specular, specular, 0.0);
        
        let alpha = if is_star {
            (color.w * 255.0)
        } else {
            (color.w * 255.0) * (1.0 - t * 0.5)
        };
        let col = Color::new(
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
            alpha as u8
        );
        
        let r = radius * (1.0 - (1.0 - t).powf(2.0) * 0.2);  
        d.draw_circle(center.x as i32, center.y as i32, r, col);
    }
}

impl RaylibRenderer {

    pub fn update_and_draw(&mut self) {
        if self.system_auto_rotation {
            self.system_rotation += self.system_rotation_speed;
        }

        if self.global_spin_enabled {
            for b in self.bodies.iter_mut() {
                b.spin_phase += b.spin_speed * self.global_spin_speed;
            }
        }

        self.camera_rotation_y = self.camera.rotation_angle;
        self.camera_rotation_x = (self.camera.position.y / self.camera.rotation_radius).clamp(-1.2, 1.2);
        self.camera_scale = 5.0 * self.camera.zoom;

        let thread = &self.thread;
        let mut d = (&mut self.rl).begin_drawing(thread);
        d.clear_background(Color::BLACK);

        let cx = self.camera_center.x;
        let cy = self.camera_center.y;
        
        d.draw_circle(cx as i32, cy as i32, 200.0, Color::DARKGRAY);
        
        if self.show_orbits {
            for body in self.bodies.iter() {
                let orbit_radius = body.distance * 100.0;
                d.draw_circle_lines(cx as i32, cy as i32, orbit_radius, Color::new(100, 100, 100, 100));
            }
        }

        let max_pitch = 1.2_f32; 
        if self.camera_rotation_x > max_pitch { self.camera_rotation_x = max_pitch; }
        if self.camera_rotation_x < -max_pitch { self.camera_rotation_x = -max_pitch; }

        let camera_distance = 400.0_f32;
        struct DrawEntry<'a> {
            depth: f32,        
            screen_pos: Vector2,
            screen_radius: f32,
            light_dir: Option<Vec3>,
            body: &'a Body,
        }

        let star_world = self.bodies.first().map(|star| {
            let star_angle = self.system_rotation * star.rotation_speed;
            Vec3::new(
                star.distance * star_angle.cos() * 100.0,
                0.0,
                star.distance * star_angle.sin() * 100.0,
            )
        });

        let mut draw_list: Vec<DrawEntry> = Vec::with_capacity(self.bodies.len());

        for (i, body) in self.bodies.iter().enumerate() {
            let angle = self.system_rotation * body.rotation_speed + body.orbit_phase;
            let world_x = body.distance * angle.cos() * 100.0;
            let world_y = 0.0_f32;
            let world_z = body.distance * angle.sin() * 100.0;

            let rx = self.camera_rotation_x;
            let ry = self.camera_rotation_y;

            let y1 = world_y * rx.cos() - world_z * rx.sin();
            let z1 = world_y * rx.sin() + world_z * rx.cos();

            let x2 = world_x * ry.cos() - z1 * ry.sin();
            let z2 = world_x * ry.sin() + z1 * ry.cos();

            let z_view = z2 + camera_distance;

            let denom = z_view.max(50.0);
            let scale = self.camera_scale / denom;
            let screen_x = x2 * scale;
            let screen_y = -y1 * scale; 
            let pos = Vector2::new(self.camera_center.x + screen_x, self.camera_center.y + screen_y);

            let light_dir = if i > 0 {
                let planet_pos = Vec3::new(world_x, world_y, world_z);
                star_world.map(|star| (star - planet_pos).normalize())
            } else {
                None
            };

            draw_list.push(DrawEntry {
                depth: z_view,
                screen_pos: pos,
                screen_radius: body.radius * 50.0,
                light_dir,
                body,
            });
        }

        draw_list.sort_by(|a, b| b.depth.partial_cmp(&a.depth).unwrap_or(std::cmp::Ordering::Equal));

        for entry in draw_list.iter() {
            let body = entry.body;
            paint_sphere(&mut d,
                        entry.screen_pos,
                        entry.screen_radius,
                        &body.color_layers,
                        &body.layer_thresholds,
                        self.camera_rotation_x,
                        self.camera_rotation_y,
                        self.shader_layers,
                        body.is_star,
                        entry.light_dir);

            if body.has_ring {
                d.draw_circle_lines(entry.screen_pos.x as i32, entry.screen_pos.y as i32, (body.radius*80.0) as f32, Color::LIGHTGRAY);
            }

            if body.has_moon {
                let moon_angle = self.system_rotation * 2.0;
                let mx = entry.screen_pos.x + (body.radius*150.0) * moon_angle.cos();
                d.draw_circle(mx as i32, entry.screen_pos.y as i32, (body.radius*15.0) as f32, Color::LIGHTGRAY);
            }

            let marker_r = body.radius * 50.0;
            let mangle = body.spin_phase;
            let mx = entry.screen_pos.x + mangle.cos() * marker_r * 0.9;
            let my = entry.screen_pos.y + mangle.sin() * marker_r * 0.9;
            d.draw_circle(mx as i32, my as i32, (marker_r*0.06).max(2.0), Color::BLACK);
        }

        let controls = vec![
            "Controls:".to_string(),
            "Camera: Arrow keys to rotate, WASD to pan".to_string(),
            format!("[H] System rotation: {}", if self.system_auto_rotation { "AUTO" } else { "MANUAL" }),
            format!("[G] Planet spin: {}", if self.global_spin_enabled { "ON" } else { "OFF" }),
            format!("[+/-] Spin speed: {:.1}x", self.global_spin_speed),
            format!("[O] Show orbits: {}", if self.show_orbits { "ON" } else { "OFF" }),
            format!("[Z/X] Shader layers: {} (change)", self.shader_layers),
            "[Mouse Wheel] Zoom in/out".to_string(),
            "[Space] Reset view".to_string(),
        ];
        
        for (i, text) in controls.iter().enumerate() {
            let y = 10 + i as i32 * 25;
            let color = if text.contains("OFF") { Color::GRAY } else { Color::WHITE };
            d.draw_text(text, 10, y, 20, color);
        }
    }
}
