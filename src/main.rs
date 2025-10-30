mod camera;
mod planet;
mod orbit;
mod renderer;
mod shaders;
mod utils;

use raylib::prelude::*;
use camera::CameraController;
use planet::{Planet, PlanetKind};
use renderer::Renderer;
use orbit::OrbitSystem;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("GraficasAndSpace - Sistema Solar 3D")
        .build();

    rl.set_target_fps(60);

    let mut camera = CameraController::new();
    let mut renderer = Renderer::new();

    let mut planets = vec![
        Planet::new("Mercurio", PlanetKind::Rocky, 5.0, 2.2, 0.45, Color::LIGHTGRAY, false, false),
        Planet::new("Venus",   PlanetKind::Rocky, 7.0, 1.6, 0.60, Color::ORANGE,    false, false),
        Planet::new("Tierra",  PlanetKind::Rocky, 9.5, 1.1, 0.70, Color::BLUE,      false, true), 
        Planet::new("Marte",   PlanetKind::Rocky, 12.0,0.9, 0.50, Color::RED,       false, false),
        Planet::new("Jupiter", PlanetKind::Gaseous,16.0,0.5, 1.30, Color::BROWN,    true,  false),
        Planet::new("Saturno", PlanetKind::Gaseous,20.0,0.35,1.00, Color::BEIGE,    true,  false),
    ];

    let mut orbits = OrbitSystem::new();

    let mut show_grid = true;
    let mut show_ui = true;
    let mut rotate_enabled = true;

    while !rl.window_should_close() {
        camera.handle_input(&mut rl);
        if rl.is_key_pressed(KeyboardKey::KEY_G) { show_grid = !show_grid; }
        if rl.is_key_pressed(KeyboardKey::KEY_U) { show_ui = !show_ui; }
        if rl.is_key_pressed(KeyboardKey::KEY_R) { rotate_enabled = !rotate_enabled; }

        let dt = rl.get_frame_time();
        orbits.advance(dt);
        for p in planets.iter_mut() {
            p.update(dt, rotate_enabled, orbits.time);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut mode3d = d.begin_mode3D(camera.camera);
            renderer.render_scene(&mut mode3d, &planets, orbits.time, show_grid, camera.camera.position);
        }

        if show_ui {
            d.draw_text("Controles: A/D rotar cam | W/S zoom | R toggle rot | G toggle grid | U toggle UI", 10, 10, 14, Color::RAYWHITE);
            d.draw_text(&format!("Tiempo: {:.2}s", orbits.time), 10, 28, 12, Color::LIGHTGRAY);
        }
    }
}
