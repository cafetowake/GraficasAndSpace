use crate::framebuffer::{Framebuffer, Color};
use nalgebra::Vector3;

pub struct Vertex3D {
    pub position: Vector3<f32>,
    pub color: Color,
}

impl Vertex3D {
    pub fn new(position: Vector3<f32>, color: Color) -> Self {
        Self { position, color }
    }
}
fn barycentric_coordinates(p: &Vector3<f32>, a: &Vector3<f32>, b: &Vector3<f32>, c: &Vector3<f32>) -> Vector3<f32> {
    let v0 = *c - *a;
    let v1 = *b - *a;
    let v2 = *p - *a;
    
    let dot00 = v0.dot(&v0);
    let dot01 = v0.dot(&v1);
    let dot02 = v0.dot(&v2);
    let dot11 = v1.dot(&v1);
    let dot12 = v1.dot(&v2);
    
    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;
    let w = 1.0 - u - v;
    
    Vector3::new(w, v, u)
}

fn is_inside_triangle(bary: &Vector3<f32>) -> bool {
    bary.x >= 0.0 && bary.y >= 0.0 && bary.z >= 0.0
}

fn interpolate_color(bary: &Vector3<f32>, c0: Color, c1: Color, c2: Color) -> Color {
    let r = (bary.x * c0.r as f32 + bary.y * c1.r as f32 + bary.z * c2.r as f32) as u8;
    let g = (bary.x * c0.g as f32 + bary.y * c1.g as f32 + bary.z * c2.g as f32) as u8;
    let b = (bary.x * c0.b as f32 + bary.y * c1.b as f32 + bary.z * c2.b as f32) as u8;
    Color::new(r, g, b)
}

fn interpolate_depth(bary: &Vector3<f32>, z0: f32, z1: f32, z2: f32) -> f32 {
    bary.x * z0 + bary.y * z1 + bary.z * z2
}
pub fn draw_triangle_filled(framebuffer: &mut Framebuffer, v0: &Vertex3D, v1: &Vertex3D, v2: &Vertex3D) {
    let p0 = Vector3::new(v0.position.x, v0.position.y, v0.position.z);
    let p1 = Vector3::new(v1.position.x, v1.position.y, v1.position.z);
    let p2 = Vector3::new(v2.position.x, v2.position.y, v2.position.z);
    
    let min_x = (p0.x.min(p1.x.min(p2.x))).floor() as i32;
    let max_x = (p0.x.max(p1.x.max(p2.x))).ceil() as i32;
    let min_y = (p0.y.min(p1.y.min(p2.y))).floor() as i32;
    let max_y = (p0.y.max(p1.y.max(p2.y))).ceil() as i32;
    
    let min_x = min_x.max(0).min(framebuffer.width() as i32 - 1);
    let max_x = max_x.max(0).min(framebuffer.width() as i32 - 1);
    let min_y = min_y.max(0).min(framebuffer.height() as i32 - 1);
    let max_y = max_y.max(0).min(framebuffer.height() as i32 - 1);
    
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vector3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            let bary = barycentric_coordinates(&point, &p0, &p1, &p2);
            
            if is_inside_triangle(&bary) {
                let depth = interpolate_depth(&bary, p0.z, p1.z, p2.z);
                let color = interpolate_color(&bary, v0.color, v1.color, v2.color);
                framebuffer.set_pixel_with_depth(x, y, depth, color);
            }
        }
    }
}

pub fn draw_triangle_wireframe(framebuffer: &mut Framebuffer, v0: &Vertex3D, v1: &Vertex3D, v2: &Vertex3D, color: Color) {
    let x0 = v0.position.x as i32;
    let y0 = v0.position.y as i32;
    let x1 = v1.position.x as i32;
    let y1 = v1.position.y as i32;
    let x2 = v2.position.x as i32;
    let y2 = v2.position.y as i32;
    
    framebuffer.draw_line(x0, y0, x1, y1, color);
    framebuffer.draw_line(x1, y1, x2, y2, color);
    framebuffer.draw_line(x2, y2, x0, y0, color);
}

