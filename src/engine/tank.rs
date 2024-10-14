use std::{f32::consts::PI, process::CommandArgs};

use bevy::{animation::transition, asset::AssetServer, math::Vec3, prelude::{BuildChildren, Commands, Component, Entity, Event, EventReader, GlobalTransform, Query, Res, Transform, With, Without}, sprite::SpriteBundle, time::Time};

use crate::player::PlayerID;

#[derive(Debug, Clone, Copy, Event)]
pub enum Instruction<const P_FLAG: u32> {
    MoveForward,
    MoveBackward,
    RotateLeft,
    RotateRight,

    SpinTurretLeft,
    SpinTurretRight,
    Shoot
}

const TANK_ROTATION_SPEED: f32 = PI / 2.;
const TURRET_ROTATION_SPEED: f32 = 3. * PI / 2.;
const TANK_MOVE_SPEED: f32 = 100.;

pub fn process_tank_instruction<const P_FLAG: u32>(
    mut tank_query: Query<(&mut Transform, &Tank), With<PlayerID<P_FLAG>>>,
    mut turret_query: Query<&mut Transform, (Without<PlayerID<P_FLAG>>, With<Turret>)>,
    mut instruction_events: EventReader<Instruction<P_FLAG>>,

    time: Res<Time>,
){
    instruction_events.read()
        .for_each(|inst| {
            // println!("{P_FLAG} - {inst:?}");

            tank_query.iter_mut()
                .for_each(|(mut transform, tank)|{
                    let transform = transform.as_mut();

                    match inst {
                        // movement
                        Instruction::RotateLeft => {
                            transform.rotate_z(-TANK_ROTATION_SPEED * time.delta_seconds());
                        },
                        Instruction::RotateRight => {
                            transform.rotate_z(TANK_ROTATION_SPEED * time.delta_seconds());
                        },
                        Instruction::MoveForward => {
                            transform.translation = transform.translation + TANK_MOVE_SPEED * transform.up() * time.delta_seconds();
                        },
                        Instruction::MoveBackward => {
                            transform.translation = transform.translation - TANK_MOVE_SPEED * transform.up() * time.delta_seconds();
                        }

                        // turret
                        Instruction::SpinTurretLeft => {
                            let mut turret_transform = turret_query.get_mut(tank.turret)
                                .expect("Tank has lost ref it's turret");
                            let turret_transform = turret_transform.as_mut();

                            turret_transform.rotate_z(-TURRET_ROTATION_SPEED * time.delta_seconds());
                        },
                        Instruction::SpinTurretRight => {
                            let mut turret_transform = turret_query.get_mut(tank.turret)
                                .expect("Tank has lost ref it's turret");
                            let turret_transform = turret_transform.as_mut();

                            turret_transform.rotate_z(TURRET_ROTATION_SPEED * time.delta_seconds());
                        },

                        Instruction::Shoot => todo!()
                    }
                });
        });
}

#[derive(Component)]
pub struct Tank{
    pub team_id: u8,
    pub turret: Entity,
}

#[derive(Component)]
pub struct Turret;

const TANK_HEIGHT: f32 = 1.;
const TURRET_HEIGHT: f32 = 3.;

pub fn create_minimal_tank(x: f32, y: f32, team_id: u8, commands: &mut Commands) -> Entity {
    let turret_id = commands.spawn((
        Turret,
        Transform{
            translation: Vec3{ x: 0., y: 0., z: TURRET_HEIGHT },
            ..Default::default()
        }
    )).id();

    let tank_id = commands.spawn((
        Tank{
            team_id: team_id,
            turret: turret_id
        },
        Transform{
            translation: Vec3{ x: x, y: y, z: TANK_HEIGHT },
            ..Default::default()
        },
        GlobalTransform::default()
    )).id();

    commands.entity(tank_id).add_child(turret_id);

    tank_id
}

pub fn create_tank(x: f32, y: f32, team_id: u8, commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let turret_id = commands.spawn((
        Turret,
        SpriteBundle{
            transform: Transform{
                translation: Vec3{ x: 0., y: 0., z: TURRET_HEIGHT },
                ..Default::default()
            },
            texture: asset_server.load("textures\\tanks\\turret.png"),
            ..Default::default()
        }
    )).id();

    let tank_id = commands.spawn((
        Tank{
            team_id: team_id,
            turret: turret_id
        },
        SpriteBundle{
            transform: Transform{
                translation: Vec3{ x: x, y: y, z: TANK_HEIGHT },
                ..Default::default()
            },
            texture: asset_server.load("textures\\tanks\\hull.png"),
            ..Default::default()
        }
    )).id();

    commands.entity(tank_id).add_child(turret_id);

    tank_id
}