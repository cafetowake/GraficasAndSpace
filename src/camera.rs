use nalgebra::{Vector3, Matrix4, Point3, Perspective3, Isometry3, Unit};
use std::f32::consts::PI;

pub struct Camera {
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub scale: f32,
    pub auto_rotate: bool,
    pub position: Vector3<f32>,
    pub target: Vector3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            rotation_x: 0.0,
            rotation_y: 0.0,
            scale: 1.0,
            auto_rotate: false,
            position: Vector3::new(0.0, 0.0, 400.0),
            target: Vector3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            fov: PI / 4.0, 
            aspect_ratio: width / height,
            near_plane: 0.1,
            far_plane: 1000.0,
        }
    }
    
    pub fn reset(&mut self) {
        self.rotation_x = 0.0;
        self.rotation_y = 0.0;
        self.scale = 1.0;
        self.auto_rotate = false;
        self.position = Vector3::new(0.0, 0.0, 400.0);
        self.target = Vector3::new(0.0, 0.0, 0.0);
    }
    
    pub fn update(&mut self, dt: f32) {
        if self.auto_rotate {
            self.rotation_y += dt * 0.5;
        }
    }
    
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let eye = Point3::from(self.position);
        let target = Point3::from(self.target);
        let up = Unit::new_normalize(self.up);
        
        Isometry3::look_at_rh(&eye, &target, &up).to_homogeneous()
    }
    
    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        let perspective = Perspective3::new(self.aspect_ratio, self.fov, self.near_plane, self.far_plane);
        perspective.to_homogeneous()
    }
    
    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        let scale_matrix = Matrix4::new_scaling(self.scale);
        let rot_x = Matrix4::from_axis_angle(&Vector3::x_axis(), self.rotation_x);
        let rot_y = Matrix4::from_axis_angle(&Vector3::y_axis(), self.rotation_y);
        
        scale_matrix * rot_y * rot_x
    }
    
    pub fn get_mvp_matrix(&self) -> Matrix4<f32> {
        let model = self.get_model_matrix();
        let view = self.get_view_matrix();
        let projection = self.get_projection_matrix();
        
        projection * view * model
    }
    
    pub fn project_vertex(&self, vertex: &Vector3<f32>, screen_width: f32, screen_height: f32) -> Vector3<f32> {
        let mvp = self.get_mvp_matrix();
        let point = Point3::from(*vertex);
        let transformed = mvp * point.to_homogeneous();
        
        if transformed.w != 0.0 {
            let ndc_x = transformed.x / transformed.w;
            let ndc_y = transformed.y / transformed.w;
            let ndc_z = transformed.z / transformed.w;
            
            let screen_x = (ndc_x + 1.0) * 0.5 * screen_width;
            let screen_y = (1.0 - ndc_y) * 0.5 * screen_height; 
            
            Vector3::new(screen_x, screen_y, ndc_z)
        } else {
            Vector3::new(-1000.0, -1000.0, 1000.0)
        }
    }
    
    pub fn set_aspect_ratio(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
    }
    
    pub fn zoom_in(&mut self) {
        self.scale *= 1.1;
    }
    
    pub fn zoom_out(&mut self) {
        self.scale /= 1.1;
    }
    
    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.rotation_y += delta_x;
        self.rotation_x += delta_y;
        
        self.rotation_x = self.rotation_x.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
    }
    
    pub fn toggle_auto_rotate(&mut self) {
        self.auto_rotate = !self.auto_rotate;
    }
}