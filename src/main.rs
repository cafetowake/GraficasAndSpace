fn main() {
<<<<<<< HEAD
    let (mut rl, thread) = raylib::init()
        .size(1366, 768)
        .title("Lab 4")
        .build();
    
    let mut model = Model::load_from_file("assets/Spaceship.obj").expect("Failed to load model");
    model.normalize(150.0);
    
    let mut renderer = Renderer::new(1366, 768);
    
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
=======
    println!("Hello, world!");
>>>>>>> parent of f788439 (change)
}
