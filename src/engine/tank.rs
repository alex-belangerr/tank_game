use bevy::{asset::AssetServer, ecs::world::Command, prelude::{Commands, Component, Entity, Event, EventReader, GlobalTransform, Query, Res, Transform, With}, sprite::SpriteBundle};

use crate::player::PlayerID;

#[derive(Debug, Clone, Copy, Event)]
pub enum Instruction<const P_FLAG: u32> {
    move_forward,
    move_backward,
    rotate_left,
    rotate_right,

    spin_turret_left,
    spin_turret_right,
    shoot
}

pub fn process_tank_instruction<const P_FLAG: u32>(
    query: Query<&mut Transform, With<PlayerID<P_FLAG>>>,
    mut instruction_events: EventReader<Instruction<P_FLAG>>
){
    for instruction in instruction_events.read(){
        println!("{P_FLAG} - {instruction:?}")
    }
}

#[derive(Component)]
pub struct Tank{
    pub team_id: u8,
    pub turret: Entity,
}

#[derive(Component)]
pub struct Turret;

pub fn create_minimal_tank(x: f32, y: f32, team_id: u8, commands: &mut Commands) -> Entity {
    let turret_id = commands.spawn((
        Turret,
        Transform{
            translation: bevy::math::Vec3 { x: x, y: y, z: 1. },
            ..Default::default()
        }
    )).id();

    commands.spawn((
        Tank{
            team_id: team_id,
            turret: turret_id
        },
        Transform{
            translation: bevy::math::Vec3 { x: x, y: y, z: 1. },
            ..Default::default()
        },
        GlobalTransform::default()
    )).id()
}

pub fn create_tank(x: f32, y: f32, team_id: u8, commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let turret_id = commands.spawn((
        Turret,
        Transform{
            translation: bevy::math::Vec3 { x: x, y: y, z: 1. },
            ..Default::default()
        }
    )).id();

    commands.spawn((
        Tank{
            team_id: team_id,
            turret: turret_id
        },
        SpriteBundle{
            transform: Transform{
                translation: bevy::math::Vec3 { x: x, y: y, z: 1. },
                ..Default::default()
            },
            texture: asset_server.load("textures\\tanks\\hull.png"),
            ..Default::default()
        }
    )).id()
}