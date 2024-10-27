use bevy::{asset::Assets, math::{Vec2, Vec3}, prelude::{Camera, EventReader, OrthographicProjection, Query, Res, Transform, With, Without}, window::WindowResized};

use super::{map::{CurrentMap, Map, WALL_SIZE}, tank::gen::Tank};



pub fn update_camera_pos(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Tank>)>,

    tank_query: Query<&Transform, With<Tank>>
){
    let tank_pos: Vec2 = tank_query.iter()
        .map(|transform| transform.translation)
        .fold(Vec2::new(0., 0.), |mut acc, next| {
            acc.x += next.x;
            acc.y += next.y;

            acc
        });

    let tank_count = tank_query.iter().count();

    let ave_pos = Vec2::new(tank_pos.x / tank_count as f32, tank_pos.y / tank_count as f32);

    camera_query.iter_mut()
        .for_each(|mut transform| {
            let transform = transform.as_mut();

            transform.translation = Vec3::new(
                ave_pos.x,
                ave_pos.y,
                0.
            );
        });
}


pub fn resize_camera(
    mut resize_reader: EventReader<WindowResized>,
    
    current_map: Res<CurrentMap>,
    map_res: Res<Assets<Map>>,

    mut query_camera: Query<(&mut OrthographicProjection, &mut Transform)>,
) {
    let Some(map) = map_res.get(
        {
            let Some(current_map) = &current_map.0 else {
                return ;
            };

            current_map
        }) else {
        return;
    };

    let Ok((mut camera_projection, mut transform)) = query_camera.get_single_mut() else {
        return;
    };
    let camera_projection = camera_projection.as_mut();
    let transform = transform.as_mut();

    let Some(window_size) = resize_reader.read().last() else {
        return ;
    };

    camera_projection.scale = match window_size.width > window_size.height {
        false => ((1. + map.dim.0 as f32) * WALL_SIZE) / window_size.width,
        true => ((1. + map.dim.1 as f32) * WALL_SIZE) / window_size.height,
    };
    transform.translation = Vec3{
        x: (map.dim.0 - 1) as f32 / 2. * WALL_SIZE,
        y: (map.dim.1 - 1) as f32 / 2. * WALL_SIZE,
        z: 1. 
    }
}