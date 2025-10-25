use crate::engine::vertex::Vertex;

pub fn generate_sphere(radius: f32, sectors: u32, stacks: u32) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=stacks {
        let phi = std::f32::consts::PI * i as f32 / stacks as f32;
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();

        for j in 0..=sectors {
            let theta = 2.0 * std::f32::consts::PI * j as f32 / sectors as f32;
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();

            let x = cos_theta * sin_phi;
            let y = cos_phi;
            let z = sin_theta * sin_phi;

            vertices.push(Vertex {
                position: [x * radius, y * radius, z * radius],
                normal: [x, y, z],
            });

            if i < stacks && j < sectors {
                let current = i * (sectors + 1) + j;
                let next = current + (sectors + 1);

                indices.push(current as u16);
                indices.push((current + 1) as u16);
                indices.push(next as u16);

                indices.push((current + 1) as u16);
                indices.push((next + 1) as u16);
                indices.push(next as u16);
            }
        }
    }

    (vertices, indices)
}

pub fn generate_ring(inner_radius: f32, outer_radius: f32, segments: u32) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=segments {
        let theta = 2.0 * std::f32::consts::PI * i as f32 / segments as f32;
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        // Inner vertex
        vertices.push(Vertex {
            position: [cos_theta * inner_radius, 0.0, sin_theta * inner_radius],
            normal: [0.0, 1.0, 0.0],
        });

        // Outer vertex
        vertices.push(Vertex {
            position: [cos_theta * outer_radius, 0.0, sin_theta * outer_radius],
            normal: [0.0, 1.0, 0.0],
        });

        if i < segments {
            let base = i * 2;
            indices.push(base as u16);
            indices.push((base + 1) as u16);
            indices.push((base + 2) as u16);

            indices.push((base + 1) as u16);
            indices.push((base + 3) as u16);
            indices.push((base + 2) as u16);
        }
    }

    (vertices, indices)
}