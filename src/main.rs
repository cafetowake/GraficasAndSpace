use glam::Vec4;
use raylib::ffi::KeyboardKey;
use raylib::prelude::Vector2;

mod engine;
use engine::renderer_raylib::{RaylibRenderer, Body};

fn main() {
    let mut renderer = RaylibRenderer::new("Solar System Shader Lab", 800, 600);

    // Sun (central star)
    renderer.add_body(Body {
        radius: 1.0,
        distance: 0.0,
        rotation_speed: 0.2,
        spin_speed: 0.5,
        spin_phase: 0.0,
        color_layers: [
            Vec4::new(1.0, 0.2, 0.0, 1.0),
            Vec4::new(1.0, 0.4, 0.0, 1.0),
            Vec4::new(1.0, 0.6, 0.0, 1.0),
            Vec4::new(1.0, 0.8, 0.2, 1.0),
        ],
        layer_thresholds: [0.25, 0.5, 0.75],
        has_ring: false,
        has_moon: false,
        is_star: true,
        orbit_phase: 0.0,
    });

    // Rocky planet + moon
    renderer.add_body(Body {
        radius: 0.5,
        distance: 2.0,
        rotation_speed: 1.0,
        spin_speed: 1.0,
        spin_phase: 0.0,
        color_layers: [
            Vec4::new(0.5, 0.2, 0.1, 1.0),
            Vec4::new(0.6, 0.3, 0.2, 1.0),
            Vec4::new(0.7, 0.4, 0.3, 1.0),
            Vec4::new(0.8, 0.5, 0.4, 1.0),
        ],
        layer_thresholds: [0.2, 0.5, 0.8],
        has_ring: false,
        has_moon: true,
        is_star: false,
        orbit_phase: 0.6,
    });

    // Gas giant + rings
    renderer.add_body(Body {
        radius: 0.8,
        distance: 4.0,
        rotation_speed: 0.3,
        spin_speed: 0.3,
        spin_phase: 0.0,
        color_layers: [
            Vec4::new(0.4, 0.6, 0.8, 1.0),
            Vec4::new(0.5, 0.7, 0.9, 1.0),
            Vec4::new(0.6, 0.8, 1.0, 1.0),
            Vec4::new(0.7, 0.9, 1.0, 1.0),
        ],
        layer_thresholds: [0.25, 0.5, 0.75],
        has_ring: true,
        has_moon: false,
        is_star: false,
        orbit_phase: 2.0,
    });

    // Extra small ice world (extra planet for rubric points)
    renderer.add_body(Body {
        radius: 0.4,
        distance: 6.0,
        rotation_speed: 0.6,
        spin_speed: 0.8,
        spin_phase: 0.0,
        color_layers: [
            Vec4::new(0.6, 0.8, 0.9, 1.0),
            Vec4::new(0.5, 0.7, 0.9, 1.0),
            Vec4::new(0.4, 0.6, 0.8, 1.0),
            Vec4::new(0.3, 0.5, 0.7, 1.0),
        ],
        layer_thresholds: [0.25, 0.5, 0.75],
        has_ring: false,
        has_moon: false,
        is_star: false,
        orbit_phase: 4.0,
    });

    // Extra rocky planet
    renderer.add_body(Body {
        radius: 0.6,
        distance: 8.0,
        rotation_speed: 0.4,
        spin_speed: 0.6,
        spin_phase: 0.0,
        color_layers: [
            Vec4::new(0.3, 0.2, 0.1, 1.0),
            Vec4::new(0.4, 0.25, 0.15, 1.0),
            Vec4::new(0.5, 0.35, 0.2, 1.0),
            Vec4::new(0.6, 0.45, 0.3, 1.0),
        ],
        layer_thresholds: [0.2, 0.5, 0.8],
        has_ring: false,
        has_moon: false,
        is_star: false,
        orbit_phase: 5.5,
    });

    // Main loop with simple input for zoom
    while !renderer.rl.window_should_close() {
        // Handle camera rotation (arrow keys) and panning (WASD)
        if renderer.rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            renderer.camera.rotate(1.5);
        }
        if renderer.rl.is_key_down(KeyboardKey::KEY_LEFT) {
            renderer.camera.rotate(-1.5);
        }
        if renderer.rl.is_key_down(KeyboardKey::KEY_UP) {
            renderer.camera.move_vertical(-1.0);
        }
        if renderer.rl.is_key_down(KeyboardKey::KEY_DOWN) {
            renderer.camera.move_vertical(1.0);
        }

        // WASD to pan the whole system on screen
        let pan_speed = 8.0_f32 * (renderer.camera_scale / 5.0);
        if renderer.rl.is_key_down(KeyboardKey::KEY_A) {
            renderer.camera_center.x -= pan_speed;
        }
        if renderer.rl.is_key_down(KeyboardKey::KEY_D) {
            renderer.camera_center.x += pan_speed;
        }
        if renderer.rl.is_key_down(KeyboardKey::KEY_W) {
            renderer.camera_center.y -= pan_speed;
        }
        if renderer.rl.is_key_down(KeyboardKey::KEY_S) {
            renderer.camera_center.y += pan_speed;
        }

        // Handle zoom with mouse wheel (use Camera API)
        let wheel_move = renderer.rl.get_mouse_wheel_move();
        if wheel_move != 0.0 {
            renderer.camera.update_zoom(wheel_move);
            renderer.camera_scale = 5.0 * renderer.camera.zoom;
            renderer.camera_scale = renderer.camera_scale.clamp(2.0, 10.0);
        }

        // keyboard controls: H toggles system auto-rotation, G toggles global spin
        if renderer.rl.is_key_pressed(KeyboardKey::KEY_H) {
            renderer.system_auto_rotation = !renderer.system_auto_rotation;
        }
        if renderer.rl.is_key_pressed(KeyboardKey::KEY_G) {
            renderer.global_spin_enabled = !renderer.global_spin_enabled;
        }
        // +/- to change global spin speed
        if renderer.rl.is_key_down(KeyboardKey::KEY_KP_ADD) || renderer.rl.is_key_down(KeyboardKey::KEY_EQUAL) {
            renderer.global_spin_speed *= 1.05;
        }
        if renderer.rl.is_key_down(KeyboardKey::KEY_KP_SUBTRACT) || renderer.rl.is_key_down(KeyboardKey::KEY_MINUS) {
            renderer.global_spin_speed /= 1.05;
        }

        // Toggle orbit display with O key
        if renderer.rl.is_key_pressed(KeyboardKey::KEY_O) {
            renderer.show_orbits = !renderer.show_orbits;
        }

        // Shader layer controls (Z decrease, X increase)
        if renderer.rl.is_key_pressed(KeyboardKey::KEY_Z) {
            if renderer.shader_layers > 1 { renderer.shader_layers -= 1; }
        }
        if renderer.rl.is_key_pressed(KeyboardKey::KEY_X) {
            if renderer.shader_layers < 4 { renderer.shader_layers += 1; }
        }

        // Reset view with Space
        if renderer.rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            renderer.camera = engine::camera::Camera::new(800, 600);
            renderer.camera_scale = 5.0;
            renderer.camera_rotation_x = 0.0;
            renderer.camera_rotation_y = 0.0;
            renderer.camera_center = Vector2::new(400.0, 300.0);
        }

        renderer.update_and_draw();
    }
}
