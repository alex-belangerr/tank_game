// #import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_sprite::{
    // mesh2d_vertex_output::VertexOutput
    mesh2d_view_bindings::globals,
    mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip},
}

struct ColourData {
    colour: vec4<f32>,
};

@group(2) @binding(0) var<uniform> primary: ColourData;
@group(2) @binding(1) var<uniform> secondary: ColourData;

@group(2) @binding(2) var color_texture: texture_2d<f32>;

fn dist(
    v1: vec3<f32>,
    v2: vec3<f32>,
) -> f32 {
    let dist: vec3<f32> = v2-v1;

    return dist.x*dist.x + dist.y*dist.y + dist.z*dist.z;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // get pixel coordinate
    let dim: vec2<u32> = textureDimensions(color_texture);
    let coord: vec2<i32> = vec2(i32(mesh.uv.x * f32(dim.x)), i32(mesh.uv.y * f32(dim.y))); 

    // get colour map value
    let colour_map: vec4<f32> = textureLoad(color_texture, coord, 0);

    let primary_colour_dist: f32 = dist(
        colour_map.rgb,
        vec3(1., 0., 0.)
    );
    let secondary_colour_dist: f32 = dist(
        colour_map.rgb,
        vec3(0., 1., 0.)
    );

    // map colour map to primary or secondary colour
    if (primary_colour_dist < secondary_colour_dist) {
        return vec4(primary.colour.rgb, colour_map.a);
    }
    return vec4(secondary.colour.rgb, colour_map.a);
}