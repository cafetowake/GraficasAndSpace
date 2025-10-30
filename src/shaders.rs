use raylib::prelude::*;

const LIGHT_POS: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

fn normalize(v: Vector3) -> Vector3 {
    let len = (v.x*v.x + v.y*v.y + v.z*v.z).sqrt();
    if len == 0.0 { Vector3::new(0.0,0.0,1.0) } else { Vector3::new(v.x/len, v.y/len, v.z/len) }
}

fn lambert(normal: Vector3, light_dir: Vector3) -> f32 {
    let n = normalize(normal);
    let l = normalize(light_dir);
    let d = (n.x*l.x + n.y*l.y + n.z*l.z).max(0.0);
    (0.15 + d * 0.85).clamp(0.0, 1.0)
}

fn specular(normal: Vector3, light_dir: Vector3, view_dir: Vector3, shininess: f32) -> f32 {
    let n = normalize(normal);
    let l = normalize(light_dir);
    let v = normalize(view_dir);

    let n_dot_l = (n.x*l.x + n.y*l.y + n.z*l.z).max(0.0);
    let rx = 2.0 * n_dot_l * n.x - l.x;
    let ry = 2.0 * n_dot_l * n.y - l.y;
    let rz = 2.0 * n_dot_l * n.z - l.z;
    let r_dot_v = (rx*v.x + ry*v.y + rz*v.z).max(0.0);
    r_dot_v.powf(shininess)
}

fn hash_noise(p: Vector3) -> f32 {
    let xi = (p.x * 10.0).floor() as i64;
    let yi = (p.y * 10.0).floor() as i64;
    let zi = (p.z * 10.0).floor() as i64;
    let mut n = xi.wrapping_mul(73856093) ^ yi.wrapping_mul(19349663) ^ zi.wrapping_mul(83492791);
    n = (n ^ (n << 13)) & 0x7fffffff;
    ((n % 1000) as f32) / 1000.0
}

pub fn color_rock(pos_on_unit: Vector3, normal: Vector3, base: Color, time: f32) -> Color {
    let p = normalize(pos_on_unit);
    let n = normalize(normal);

    let v = base.color_normalize();
    let br = v.x; let bg = v.y; let bb = v.z;

    let tone0 = (br, bg, bb);
    let tone1 = (br * 0.85, bg * 0.78, bb * 0.72); 
    let tone2 = (br * 0.6,  bg * 0.55, bb * 0.5);  
    let tone3 = ((br + 1.0) * 0.45, (bg + 1.0) * 0.4, (bb + 1.0) * 0.4); 

    let lat = p.y;
    let lon = p.x.atan2(p.z);
    let noise = hash_noise(Vector3::new(lat*4.0 + time*0.05, lon*2.0, p.z*3.0));

    let mut w0 = (0.45 + 0.45 * (lat*2.0 + time*0.05).sin()).clamp(0.0, 1.0);
    let mut w1 = (noise * 0.95).clamp(0.0, 1.0) * 0.95;
    let mut w2 = (1.0 - lat.abs()) * 0.6;
    let mut w3 = (lat.abs()).powf(2.0) * 0.7;

    let s = w0 + w1 + w2 + w3 + 1e-6;
    w0 /= s; w1 /= s; w2 /= s; w3 /= s;

    let mut r = tone0.0*w0 + tone1.0*w1 + tone2.0*w2 + tone3.0*w3;
    let mut g = tone0.1*w0 + tone1.1*w1 + tone2.1*w2 + tone3.1*w3;
    let mut b = tone0.2*w0 + tone1.2*w1 + tone2.2*w2 + tone3.2*w3;

    let light_dir = Vector3::new(LIGHT_POS.x - p.x, LIGHT_POS.y - p.y, LIGHT_POS.z - p.z);
    let diff = lambert(n, light_dir);
    let spec = specular(n, light_dir, Vector3::new(0.0, 0.0, 1.0), 30.0) * 0.25;

    r = (r * (0.5 + diff * 0.9) + spec).clamp(0.0, 1.0);
    g = (g * (0.5 + diff * 0.9) + spec*0.9).clamp(0.0, 1.0);
    b = (b * (0.5 + diff * 0.9) + spec*0.7).clamp(0.0, 1.0);

    let view_dir = Vector3::new(0.0, 0.0, 1.0);
    let rim = (1.0 - (n.x*view_dir.x + n.y*view_dir.y + n.z*view_dir.z).max(0.0)).powf(2.0) * 0.15;
    r = (r + rim).clamp(0.0, 1.0);
    g = (g + rim * 0.9).clamp(0.0, 1.0);
    b = (b + rim * 0.8).clamp(0.0, 1.0);

    Color::new((r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8, 255)
}

pub fn color_gas(pos_on_unit: Vector3, normal: Vector3, base: Color, time: f32) -> Color {
    let p = normalize(pos_on_unit);
    let n = normalize(normal);

    let v = base.color_normalize();
    let br = v.x; let bg = v.y; let bb = v.z;

    let c1 = (br * 0.85 + 0.05, bg * 0.85 + 0.05, bb * 0.85 + 0.05);
    let c2 = (br * 0.25 + 0.5,  bg * 0.25 + 0.35, bb * 0.55 + 0.2);
    let c3 = (br * 0.6 + 0.2,  bg * 0.7 + 0.1,  bb * 0.9 + 0.05);
    let c4 = (br * 0.95 + 0.02, bg * 0.9 + 0.02, bb * 0.9 + 0.02);

    let bands = ((p.y * 6.0 + time * 0.6).sin() * 0.5 + 0.5).clamp(0.0, 1.0);

    let mut r = c1.0 * (1.0 - bands) + c2.0 * bands;
    let mut g = c1.1 * (1.0 - bands) + c2.1 * bands;
    let mut b = c1.2 * (1.0 - bands) + c2.2 * bands;

    let t2 = ((p.x * 10.0 + time * 0.5).sin() * 0.5 + 0.5) * 0.15;
    r = r * (1.0 - t2) + c3.0 * t2;
    g = g * (1.0 - t2) + c3.1 * t2;
    b = b * (1.0 - t2) + c3.2 * t2;

    let t3 = hash_noise(Vector3::new(p.x*8.0 + time*0.2, p.y*8.0, p.z*8.0));
    r = r * (1.0 - 0.08) + c4.0 * 0.08 * t3;
    g = g * (1.0 - 0.08) + c4.1 * 0.08 * t3;
    b = b * (1.0 - 0.08) + c4.2 * 0.08 * t3;

    let light_dir = Vector3::new(LIGHT_POS.x - p.x, LIGHT_POS.y - p.y, LIGHT_POS.z - p.z);
    let diff = lambert(n, light_dir);
    let spec = specular(n, light_dir, Vector3::new(0.0,0.0,1.0), 24.0) * 0.15;

    r = (r * (0.6 + diff * 1.0) + spec).clamp(0.0, 1.0);
    g = (g * (0.6 + diff * 1.0) + spec).clamp(0.0, 1.0);
    b = (b * (0.6 + diff * 1.0) + spec).clamp(0.0, 1.0);

    let view_dir = Vector3::new(0.0, 0.0, 1.0);
    let rim = (1.0 - (n.x*view_dir.x + n.y*view_dir.y + n.z*view_dir.z).max(0.0)).powf(3.0) * 0.08;
    r = (r + rim).clamp(0.0, 1.0);
    g = (g + rim * 0.9).clamp(0.0, 1.0);
    b = (b + rim * 0.7).clamp(0.0, 1.0);

    Color::new((r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8, 255)
}
