# Solar System CPU Rendering Lab

A Rust implementation of a solar system visualization using Raylib. This project demonstrates how to create visually interesting celestial bodies without using textures or GPU shaders, implementing all visual effects through CPU-based layered rendering.

## Features

- CPU-based 4-layer color system for each celestial body
- Implemented bodies:
  - Star (sun) with gradient layers
  - Rocky planet with moon
  - Gas giant with rings
- Automatic and manual rotation controls
- Individual planet spin and system rotation
- Interactive camera with zoom
- No textures or shaders - all visual effects achieved through CPU rendering

## Screenshots

Placeholders for screenshots (replace with actual images when submitting):

![Full system view placeholder](screenshots/full_system.png)

![Close-up layered planet placeholder](screenshots/planet_closeup.png)

![Gas giant with rings placeholder](screenshots/gas_giant.png)

![Moon orbiting placeholder](screenshots/moon.png)

## Implementation Details

### CPU Layer System

Instead of GPU shaders, this implementation maps shader complexity to CPU-side rendering:

- Each celestial body uses 4 distinct color layers
- Layers are rendered using concentric circles
- Layer thresholds control color transitions
- Orientation markers show rotation

### Planet Features

1. Star (Sun)
   - 4 color layers from deep orange to bright yellow
   - Automatic spin rotation
   
2. Rocky Planet
   - 4 distinct color layers
   - Orbiting moon
   - Individual spin control

3. Gas Giant
   - 4 atmospheric color layers
   - Ring system using circle outlines
   - Customizable spin speed

### Technical Specifications

- Built with Rust and Raylib
- CPU-based procedural rendering
- No GPU shaders or textures required
- Real-time animation and controls

## Controls

- Mouse wheel: Zoom in/out
- H: Toggle system auto-rotation
- G: Toggle planet spin
- +/-: Adjust global spin speed
- O: Show/hide orbit paths
- Space: Reset view
- Arrow keys: rotate view (horizontal and vertical pitch)
- W/A/S/D: pan the whole system on screen
- Z / X: decrease / increase number of shader/color layers (1..4)

## Build and Run

```bash
cargo run --release
```

## Project Structure

```text
src/
  ├── engine/
  │   ├── mod.rs               # Module exports
  │   └── renderer_raylib.rs   # Raylib-based renderer
  └── main.rs                  # Application entry and body setup
```

## Dependencies

- raylib: Window management and 2D rendering
- glam: Minimal vector math for colors
- Removed: wgpu, winit, bytemuck (no longer needed)

## Notes about development

This repository was developed interactively with the assistance of an AI pair-programmer. The AI helped refactor input handling, implement camera controls (pitch/yaw, panning, zoom), and adapt the CPU-based layered rendering system to allow a configurable number of color layers per body. The final code and decisions (sensitivity values, clamp ranges, key bindings) were chosen to provide a usable, demonstrable experience for the lab rubric.

Placeholders in `screenshots/` should be replaced by actual PNG/JPG images before final submission. The HUD shows available keys and the current shader layer count for easy screenshots.
