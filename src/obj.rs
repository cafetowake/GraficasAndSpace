use nalgebra::Vector3;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Face {
    pub indices: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector3::new(x, y, z),
        }
    }
}

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let reader = BufReader::new(file);
        
        let mut model = Model::new();
        
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let line = line.trim();
            
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            match parts[0] {
                "v" => {
                    if parts.len() >= 4 {
                        let x: f32 = parts[1].parse()
                            .map_err(|_| format!("Invalid x coordinate: {}", parts[1]))?;
                        let y: f32 = parts[2].parse()
                            .map_err(|_| format!("Invalid y coordinate: {}", parts[2]))?;
                        let z: f32 = parts[3].parse()
                            .map_err(|_| format!("Invalid z coordinate: {}", parts[3]))?;
                        
                        model.vertices.push(Vertex::new(x, y, z));
                    }
                }
                "f" => {
                    let mut indices = Vec::new();
                    
                    for i in 1..parts.len() {
                        let vertex_data = parts[i].split('/').next().unwrap();
                        let index: usize = vertex_data.parse()
                            .map_err(|_| format!("Invalid face index: {}", vertex_data))?;
                        
                        if index > 0 {
                            indices.push(index - 1);
                        }
                    }
                    
                    if indices.len() >= 3 {
                        model.faces.push(Face { indices: vec![indices[0], indices[1], indices[2]] });
                        
                        if indices.len() == 4 {
                            model.faces.push(Face { indices: vec![indices[0], indices[2], indices[3]] });
                        }
                    }
                }
                _ => {}
            }
        }
        
        Ok(model)
    }
    
    pub fn get_bounding_box(&self) -> (Vector3<f32>, Vector3<f32>) {
        if self.vertices.is_empty() {
            return (Vector3::zeros(), Vector3::zeros());
        }
        
        let mut min = self.vertices[0].position;
        let mut max = self.vertices[0].position;
        
        for vertex in &self.vertices {
            let pos = vertex.position;
            min.x = min.x.min(pos.x);
            min.y = min.y.min(pos.y);
            min.z = min.z.min(pos.z);
            max.x = max.x.max(pos.x);
            max.y = max.y.max(pos.y);
            max.z = max.z.max(pos.z);
        }
        
        (min, max)
    }
    
    pub fn get_center(&self) -> Vector3<f32> {
        let (min, max) = self.get_bounding_box();
        (min + max) * 0.5
    }
    
    pub fn get_scale(&self) -> f32 {
        let (min, max) = self.get_bounding_box();
        let size = max - min;
        size.x.max(size.y).max(size.z)
    }
    
    pub fn normalize(&mut self, target_size: f32) {
        let center = self.get_center();
        let current_scale = self.get_scale();
        
        if current_scale > 0.0 {
            let scale_factor = target_size / current_scale;
            
            for vertex in &mut self.vertices {
                vertex.position = (vertex.position - center) * scale_factor;
            }
        }
    }
}