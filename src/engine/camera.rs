use bevy::{math::{Vec2, Vec3}, prelude::{Camera, Query, Transform, With, Without}};

use super::tank::gen::Tank;



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