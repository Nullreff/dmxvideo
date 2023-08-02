#import bevy_sprite::mesh2d_view_bindings   globals
#import bevy_sprite::mesh2d_bindings        mesh
#import bevy_sprite::mesh2d_functions       mesh2d_position_local_to_clip

// TODO use common view binding
#import bevy_render::view View

@group(0) @binding(0)
var<uniform> view: View;

@group(1) @binding(0)
var data: array<u8, 512>;

struct Vertex {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh2d_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.color = vec4(1.0, 1.0, 1.0, 1.0);
    return out;
}

struct FragmentInput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    //let t = 0.05 * (0.85 + sin(5.0 * globals.time));
    //return mix(vec4(1.0,1.0,1.0,1.0), vec4(1.0,1.0,1.0,1.0), smoothstep(t, t+0.01, 0));
    let width = view.viewport[2];
    let height = view.viewport[3];
    let color = vec4(data[0], data[1], data[2], 1.0);
    return color;
    //return vec4(input.position[0] / width, input.position[1] / height, 0.0, 1.0);
}