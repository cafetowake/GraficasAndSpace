use raylib::prelude::*;

pub fn vec3_len(v: Vector3) -> f32 {
    (v.x*v.x + v.y*v.y + v.z*v.z).sqrt()
}
