mod obj;
mod framebuffer;
mod triangle;
mod camera;

use obj::Model;
use framebuffer::{Framebuffer, Color};
use triangle::{Vertex3D, draw_triangle_filled, draw_triangle_wireframe};
use camera::Camera;
use raylib::prelude::*;

struct Renderer {
    framebuffer: Framebuffer,
    camera: Camera,
    pub show_wireframe: bool,
}

impl Renderer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            framebuffer: Framebuffer::new(width, height),
            camera: Camera::new(width as f32, height as f32),
            show_wireframe: false,
        }
    }
    
    fn clear(&mut self) {
        self.framebuffer.clear(Color::BLACK);
    }
    
    fn render_model(&mut self, model: &Model) {
        let screen_width = self.framebuffer.width() as f32;
        let screen_height = self.framebuffer.height() as f32;
        
        for face in &model.faces {
            if face.indices.len() >= 3 {
                let v0_world = model.vertices[face.indices[0]].position;
                let v1_world = model.vertices[face.indices[1]].position;
                let v2_world = model.vertices[face.indices[2]].position;
                
                let v0_screen = self.camera.project_vertex(&v0_world, screen_width, screen_height);
                let v1_screen = self.camera.project_vertex(&v1_world, screen_width, screen_height);
                let v2_screen = self.camera.project_vertex(&v2_world, screen_width, screen_height);
                
                if v0_screen.z > 1.0 || v1_screen.z > 1.0 || v2_screen.z > 1.0 ||
                   v0_screen.z < -1.0 || v1_screen.z < -1.0 || v2_screen.z < -1.0 {
                    continue;
                }
                
                let vertex0 = Vertex3D::new(v0_screen, Color::YELLOW);
                let vertex1 = Vertex3D::new(v1_screen, Color::YELLOW);
                let vertex2 = Vertex3D::new(v2_screen, Color::YELLOW);
                
                if self.show_wireframe {
                    draw_triangle_wireframe(&mut self.framebuffer, &vertex0, &vertex1, &vertex2, Color::WHITE);
                } else {
                    draw_triangle_filled(&mut self.framebuffer, &vertex0, &vertex1, &vertex2);
                }
            }
        }
    }
    
    fn render_to_raylib(&self, d: &mut RaylibDrawHandle) {
        let buffer = self.framebuffer.buffer();
        
        for (i, pixel) in buffer.iter().enumerate() {
            if pixel.r != 0 || pixel.g != 0 || pixel.b != 0 {
                let x = (i % self.framebuffer.width()) as i32;
                let y = (i / self.framebuffer.width()) as i32;
                d.draw_pixel(x, y, pixel.to_raylib_color());
            }
        }
    }
    
    fn toggle_wireframe(&mut self) {
        self.show_wireframe = !self.show_wireframe;
    }
    
    fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("3D Model Viewer - Lab 4")
        .build();
    
    let mut model = Model::load_from_file("assets/Spaceship.obj").expect("Failed to load model");
    model.normalize(150.0);
    
    let mut renderer = Renderer::new(800, 600);
    
    println!("Controls:");
    println!("Arrow Keys: Rotate | +/-: Zoom | R: Reset | Space: Auto-rotate | W: Toggle wireframe");
    
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        
        {
            let camera = renderer.get_camera_mut();
            
            if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                camera.rotate(-2.0 * dt, 0.0);
            }
            if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                camera.rotate(2.0 * dt, 0.0);
            }
            if rl.is_key_down(KeyboardKey::KEY_UP) {
                camera.rotate(0.0, -2.0 * dt);
            }
            if rl.is_key_down(KeyboardKey::KEY_DOWN) {
                camera.rotate(0.0, 2.0 * dt);
            }
            
            if rl.is_key_pressed(KeyboardKey::KEY_KP_ADD) || rl.is_key_pressed(KeyboardKey::KEY_EQUAL) {
                camera.zoom_in();
            }
            if rl.is_key_pressed(KeyboardKey::KEY_KP_SUBTRACT) || rl.is_key_pressed(KeyboardKey::KEY_MINUS) {
                camera.zoom_out();
            }
            
            if rl.is_key_pressed(KeyboardKey::KEY_R) {
                camera.reset();
            }
            if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                camera.toggle_auto_rotate();
            }
            
            camera.update(dt);
        }
        
        if rl.is_key_pressed(KeyboardKey::KEY_W) {
            renderer.toggle_wireframe();
        }
        
        renderer.clear();
        renderer.render_model(&model);
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(raylib::color::Color::BLACK);
        
        renderer.render_to_raylib(&mut d);
        
        let mode_text = if renderer.show_wireframe { "Wireframe" } else { "Filled" };
        d.draw_text(&format!("Mode: {} | Triangles: {}", mode_text, model.faces.len()), 
                   10, 10, 20, raylib::color::Color::WHITE);
        d.draw_text("Arrow Keys: Rotate | +/-: Zoom | R: Reset | Space: Auto | W: Wireframe", 
                   10, 30, 16, raylib::color::Color::LIGHTGRAY);
    }
}
