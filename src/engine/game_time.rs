use bevy::{prelude::{Res, ResMut, Resource}, time::Time};


#[derive(Debug, Resource)]
pub struct DeltaTime(pub f32);

pub fn update_delta_time(
    time: Res<Time>,
    mut delta_time: ResMut<DeltaTime>
) {
    let delta_time = delta_time.as_mut();

    delta_time.0 = time.delta_seconds();
}