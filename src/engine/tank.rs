//! This module handles the gameplay mechanics for tanks in a Bevy-based game,
//! including their creation, movement, turret control, and associated instructions.

use std::f32::consts::PI;

use bevy::{asset::AssetServer, math::Vec3, prelude::{BuildChildren, Commands, Component, Entity, Event, EventReader, GlobalTransform, Query, Res, Transform, With, Without}, sprite::SpriteBundle, time::Time};

use crate::player::PlayerID;

/// Represents instructions event for controlling a tank's movement and turret actions.
///
/// The Enum abstraction exists to handle a means of converting different inputs into a common instruction.
/// 
/// # Variants
/// - `MoveForward`: Instructs the tank to move forward.
/// - `MoveBackward`: Instructs the tank to move backward.
/// - `RotateLeft`: Instructs the tank to rotate left.
/// - `RotateRight`: Instructs the tank to rotate right.
/// - `SpinTurretLeft`: Instructs the turret to spin left.
/// - `SpinTurretRight`: Instructs the turret to spin right.
/// - `Shoot`: Instructs the tank to shoot.
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

/// Processes tank instructions for movement and turret control.
///
/// # Parameters
/// - `P_FLAG`: A constant representing the player ID flag.
/// - `tank_query`: A query for the tank's transform and tank components, filtered by the player ID.
/// - `turret_query`: A query for the turret's transform component, filtered by turrets that don't match the player ID.
/// - `instruction_events`: A reader for processing instruction events.
/// - `time`: A resource providing delta time for smooth frame-based calculations.
///
/// The function handles different `Instruction` variants:
/// - **RotateLeft**: Rotates the tank left.
/// - **RotateRight**: Rotates the tank right.
/// - **MoveForward**: Moves the tank forward.
/// - **MoveBackward**: Moves the tank backward.
/// - **SpinTurretLeft**: Rotates the turret left.
/// - **SpinTurretRight**: Rotates the turret right.
/// - **Shoot**: (Not implemented yet).
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

/// Represents a tank in the game.
///
/// # Fields
/// - `team_id`: The ID of the team that the tank belongs to.
/// - `turret`: The entity associated with the tank's turret.
#[derive(Component)]
pub struct Tank{
    pub team_id: u8,
    pub turret: Entity,
}

/// Represents a turret in the game.
///
/// This component is used to mark entities that act as turrets, which can rotate independently from the tank.
#[derive(Component)]
pub struct Turret;

const TANK_HEIGHT: f32 = 1.;
const TURRET_HEIGHT: f32 = 3.;

/// Creates a minimal tank entity at the specified position with a turret and assigns it to a team.
///
/// # Parameters
/// - `x`: The x-coordinate of the tank's position.
/// - `y`: The y-coordinate of the tank's position.
/// - `team_id`: The ID of the team that the tank belongs to.
/// - `commands`: A mutable reference to the `Commands` struct used to spawn and manage entities.
///
/// # Returns
/// The `Entity` ID of the created tank.
///
/// The function spawns both a tank and its associated turret, setting their initial positions 
/// and linking the turret as a child of the tank.
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

/// Creates a tank entity at the specified position with a turret and assigns it to a team,
/// loading the appropriate textures for rendering.
///
/// # Parameters
/// - `x`: The x-coordinate of the tank's position.
/// - `y`: The y-coordinate of the tank's position.
/// - `team_id`: The ID of the team that the tank belongs to.
/// - `commands`: A mutable reference to the `Commands` struct used to spawn and manage entities.
/// - `asset_server`: A resource reference to the `AssetServer` for loading asset textures.
///
/// # Returns
/// The `Entity` ID of the created tank.
///
/// This function spawns both a tank and its associated turret with the specified textures,
/// sets their initial positions, and links the turret as a child of the tank.
pub fn create_tank(x: f32, y: f32, team_id: u8, commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    // todo!() - not worth it but could reduce repetition to run `create_minimal_tank` and just add sprite & texture on top
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