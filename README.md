# Lab 4 - 3D Model Viewer

A 3D model viewer implementation in Rust featuring manual triangle rasterization and framebuffer rendering. This project loads OBJ models and renders them using software rasterization techniques.

## Features

- **OBJ Model Loading**: Load 3D models from Wavefront OBJ files
- **Manual Triangle Rasterization**: Custom triangle drawing using barycentric coordinates  
- **Framebuffer Rendering**: Software pixel manipulation with depth buffer
- **3D Camera System**: 3D transformations with Model-View-Projection matrices
- **Wireframe Mode**: Toggle between filled triangles and wireframe view
- **Interactive Controls**: Real-time rotation, zoom, and auto-rotation

## Controls

| Key | Action |
|--------|--------|
| **Arrow Keys** | Rotate object |
| **+/-** | Zoom in/out |
| **R** | Reset position |
| **Space** | Toggle auto-rotation |
| **W** | Toggle wireframe mode |
| **ESC** | Exit |

## Implementation

### Manual Rendering Pipeline
- **Vertex Transformation**: 3D to 2D projection using MVP matrices
- **Triangle Rasterization**: Barycentric coordinate-based pixel filling
- **Depth Testing**: Z-buffer for correct triangle ordering
- **Framebuffer**: Custom pixel buffer with manual pixel manipulation

### Project Structure
```
src/
├── main.rs          # Main application and render loop
├── obj.rs           # OBJ file loader and parser
├── framebuffer.rs   # Custom framebuffer with depth testing
├── triangle.rs      # Manual triangle rasterization
├── camera.rs        # 3D camera with MVP transformations
assets/
└── Spaceship.obj    # 3D model file
```

## Usage

```bash
cargo run
```

## Requirements Fulfilled

✅ **OBJ Model Loading**: Loads the Spaceship.obj model  
✅ **Manual Triangle Rendering**: Custom triangle rasterization  
✅ **Face Processing**: Manually processes faces and vertex indices  
✅ **Screen Positioning**: Model centered and properly sized  
✅ **Interactive Controls**: Rotation, zoom, and wireframe toggle  

## Dependencies

- **Rust 2021+**
- **raylib**: Window management and input handling
- **nalgebra**: 3D mathematics and transformations

---

*Lab 4 - Computer Graphics Course*
