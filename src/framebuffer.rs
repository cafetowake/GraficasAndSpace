
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };
    pub const CYAN: Color = Color { r: 0, g: 255, b: 255 };
    pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255 };
    
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    
    pub fn to_raylib_color(&self) -> raylib::color::Color {
        raylib::color::Color::new(self.r, self.g, self.b, 255)
    }
}

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<Color>,
    depth_buffer: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let pixel_count = width * height;
        Self {
            width,
            height,
            buffer: vec![Color::BLACK; pixel_count],
            depth_buffer: vec![f32::INFINITY; pixel_count],
        }
    }
    
    pub fn clear(&mut self, color: Color) {
        for pixel in &mut self.buffer {
            *pixel = color;
        }
        for depth in &mut self.depth_buffer {
            *depth = f32::INFINITY;
        }
    }
    
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = (y as usize * self.width) + x as usize;
            self.buffer[index] = color;
        }
    }
    
    pub fn set_pixel_with_depth(&mut self, x: i32, y: i32, z: f32, color: Color) -> bool {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = (y as usize * self.width) + x as usize;
            if z < self.depth_buffer[index] {
                self.depth_buffer[index] = z;
                self.buffer[index] = color;
                return true;
            }
        }
        false
    }
    
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Color> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = (y as usize * self.width) + x as usize;
            Some(self.buffer[index])
        } else {
            None
        }
    }
    
    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: Color) {
        let steep = (y1 - y0).abs() > (x1 - x0).abs();
        
        if steep {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        }
        
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }
        
        let dx = x1 - x0;
        let dy = (y1 - y0).abs();
        let mut error = dx / 2;
        let ystep = if y0 < y1 { 1 } else { -1 };
        let mut y = y0;
        
        for x in x0..=x1 {
            if steep {
                self.set_pixel(y, x, color);
            } else {
                self.set_pixel(x, y, color);
            }
            
            error -= dy;
            if error < 0 {
                y += ystep;
                error += dx;
            }
        }
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
    
    pub fn buffer(&self) -> &[Color] {
        &self.buffer
    }
    
}