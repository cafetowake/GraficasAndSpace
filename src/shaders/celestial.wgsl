struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec3<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

struct ModelUniform {
    model: mat4x4<f32>,
    color_layers: array<vec4<f32>, 4>,
    layer_thresholds: array<f32, 3>,
    time: f32,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> model: ModelUniform;

@vertex
fn vs_main(
    model_vertex: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    
    let world_position = model.model * vec4<f32>(model_vertex.position, 1.0);
    out.world_position = world_position.xyz;
    out.world_normal = normalize((model.model * vec4<f32>(model_vertex.normal, 0.0)).xyz);
    out.clip_position = camera.view_proj * world_position;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    
    // Basic lighting calculation
    let diffuse = max(dot(normal, light_dir), 0.0);
    
    // Height-based color layering with noise
    let height = (in.world_position.y + 1.0) * 0.5;
    let noise = sin(in.world_position.x * 10.0 + in.world_position.z * 10.0 + model.time) * 0.1;
    let adjusted_height = height + noise;
    
    var color = model.color_layers[3];
    if (adjusted_height < model.layer_thresholds[0]) {
        color = model.color_layers[0];
    } else if (adjusted_height < model.layer_thresholds[1]) {
        color = model.color_layers[1];
    } else if (adjusted_height < model.layer_thresholds[2]) {
        color = model.color_layers[2];
    }
    
    // Apply lighting
    let final_color = color.rgb * (diffuse * 0.8 + 0.2);
    return vec4<f32>(final_color, color.a);
}