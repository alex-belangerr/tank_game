use std::time::Duration;

use bevy::{
    asset::AssetServer, math::{Quat, Vec2, Vec3}, prelude::{Commands, Component, Entity, Event, EventReader, GlobalTransform, Query, Res, Transform, With}, sprite::SpriteBundle
};
use bevy_rapier2d::{plugin::RapierContext, prelude::{Collider, QueryFilter, ShapeCastOptions}};

use crate::engine::{game_time::DeltaTime, map::Wall};

use super::gen::{GunState, Tank, Turret};


const BULLET_HEIGHT: f32 = 0.;
const BULLET_SPEED: f32 = 400.;

#[derive(Event, Clone, Copy)]
pub struct NewBullet{
    pub start_pos: Vec3,
    pub dir: Quat,
    pub source: Entity
}

#[derive(Component, Clone, Copy)]
pub struct Bullet(pub Entity);

pub fn update_bullet_pos(
    mut bullet_query: Query<&mut Transform, With<Bullet>>,
    time: Res<DeltaTime>
){
    bullet_query.iter_mut()
        .for_each(|mut transform| {
            let transform = transform.as_mut();

            transform.translation = transform.translation + transform.up() * time.0 * BULLET_SPEED;
        });
}
pub fn create_bullet_minimal(
    mut commands: Commands,
    mut event_reader: EventReader<NewBullet>
) {
    for NewBullet{start_pos, dir, source} in event_reader.read(){
        commands.spawn((
            Bullet(*source),
            Transform{
                translation: Vec3{
                    x: start_pos.x,
                    y: start_pos.y,
                    z: BULLET_HEIGHT,
                },
                rotation: *dir,
                ..Default::default()
            },
            GlobalTransform::default(),
        ));
    }
}
pub fn create_bullet(mut commands: Commands, mut event_reader: EventReader<NewBullet>, asset_server: Res<AssetServer>){
    for NewBullet{start_pos, dir, source} in event_reader.read(){
        commands.spawn((
            Bullet(*source),
            SpriteBundle{
                transform: Transform{
                    translation: Vec3{
                        x: start_pos.x,
                        y: start_pos.y,
                        z: BULLET_HEIGHT,
                    },
                    rotation: *dir,
                    ..Default::default()
                },
                texture: asset_server.load("textures\\tanks\\bullet.png"),
                ..Default::default()
            }
        ));
    }
}

pub fn bullet_collision(
    mut commands: Commands,

    bullet_query: Query<(&Bullet, &Transform, Entity)>,
    tank_query: Query<&Tank>,
    wall_query: Query<(), With<Wall>>,

    rapier_context: Res<RapierContext>,
) {
    // todo!() Replace bullet_query with a parallel iter
    for (bullet, transform, bullet_entity) in &bullet_query{
        let pos = transform.translation;
        let rot = 0.;
        

        let cast_shape = {
            let shape = Collider::ball(7.);
            let shape_vel = Vec2::ONE;
            let filter = {
                let mut filter = QueryFilter::default();

                filter.exclude_collider = Some(bullet.0);

                filter
            };
            let options = ShapeCastOptions {
                max_time_of_impact: 0.0,
                target_distance: 0.0,
                stop_at_penetration: true,
                compute_impact_geometry_on_penetration: true,
            };
            
            rapier_context.cast_shape(
                Vec2::new(pos.x, pos.y),
                rot,
                shape_vel,
                &shape,
                options,
                filter
            )
        };

        if let Some((hit_entity, _hit)) = cast_shape {
            match (wall_query.get(hit_entity), tank_query.get(hit_entity)) {
                (Ok(_), _) => {},//do nothing
                (_, Ok(tank)) => {
                    commands.entity(tank.turret).despawn();
                    commands.entity(hit_entity).despawn();
                    //change win state
                },
                _ => panic!("Invalid entity - entity shouldn't have both Tank and Wall component"),
            }

            commands.entity(bullet_entity).despawn();
        }

    }
}

pub fn reload_gun(mut turret_query: Query<&mut Turret>, time: Res<DeltaTime>) {
    turret_query.iter_mut()
        .for_each(|mut turret| {
            let turret = turret.as_mut();
            if let GunState::Reload(timer) = &mut turret.0 {
                timer.tick(Duration::from_millis((time.0 * 1000.) as u64));

                if timer.finished() {
                    turret.0 = GunState::Ready;
                }
            };
        })
}