use std::f32::consts::PI;

use bevy::{prelude::{Event, EventReader, Query, Res, Transform, With, Without}, time::Time};

use crate::player::PlayerID;

use super::gen::{Tank, Turret};


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
