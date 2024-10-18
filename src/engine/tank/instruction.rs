use std::f32::consts::PI;

use bevy::{math::{Vec2, Vec3}, prelude::{Entity, Event, EventReader, EventWriter, GlobalTransform, Query, Res, Transform, With, Without}, time::Time};
use bevy_rapier2d::{plugin::RapierContext, prelude::{Collider, QueryFilter, ShapeCastOptions}};

use crate::player::PlayerID;

use super::{bullet::NewBullet, gen::{GunState, Tank, Turret, TANK_SIZE}};


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
    mut tank_query: Query<(&mut Transform, &Tank, Entity), With<PlayerID<P_FLAG>>>,
    mut turret_query: Query<(&mut Transform, &GlobalTransform, &mut Turret), (Without<PlayerID<P_FLAG>>, With<Turret>)>,
    mut instruction_events: EventReader<Instruction<P_FLAG>>,

    mut new_bullet: EventWriter<NewBullet>,

    time: Res<Time>,

    rapier_context: Res<RapierContext>,

    // mut gizmos: Gizmos
){
    instruction_events.read()
        .for_each(|inst| {
            // println!("{P_FLAG} - {inst:?}");

            tank_query.iter_mut()
                .for_each(|(mut transform, tank, player_entity)|{
                    let transform = transform.as_mut();

                    match inst {
                        // movement
                        Instruction::RotateLeft => update_rotation::<false>(
                            transform,
                            -TANK_ROTATION_SPEED * time.delta_seconds(),

                            &rapier_context,
                            player_entity,

                            // &mut gizmos
                        ),
                        Instruction::RotateRight => update_rotation::<false>(
                            transform,
                            TANK_ROTATION_SPEED * time.delta_seconds(),

                            &rapier_context,
                            player_entity,

                            // &mut gizmos
                        ),
                        Instruction::MoveForward => {
                            transform.translation = new_move_pos::<false>(
                                transform.translation,
                                transform.up().as_vec3(),
                                time.delta_seconds(),

                                &rapier_context,
                                player_entity,

                                // &mut gizmos
                            );
                        },
                        Instruction::MoveBackward => {
                            transform.translation = new_move_pos::<false>(
                                transform.translation,
                                transform.down().as_vec3(),
                                time.delta_seconds(),

                                &rapier_context,
                                player_entity,

                                // &mut gizmos
                            );
                        }

                        // turret
                        Instruction::SpinTurretLeft => {
                            let mut turret_transform = turret_query.get_mut(tank.turret)
                                .expect("Tank has lost ref it's turret");
                            let turret_transform = turret_transform.0.as_mut();

                            turret_transform.rotate_z(-TURRET_ROTATION_SPEED * time.delta_seconds());
                        },
                        Instruction::SpinTurretRight => {
                            let mut turret_transform = turret_query.get_mut(tank.turret)
                                .expect("Tank has lost ref it's turret");
                            let turret_transform = turret_transform.0.as_mut();

                            turret_transform.rotate_z(TURRET_ROTATION_SPEED * time.delta_seconds());
                        },

                        Instruction::Shoot => {
                            let (
                                _transform,
                                global_transform,
                                mut turret
                            ) = turret_query.get_mut(tank.turret)
                                .expect("Tank lost reference to turret");
                            let turret = turret.as_mut();

                            if let GunState::Ready = turret.0 {
                                new_bullet.send(NewBullet{
                                    start_pos: transform.translation,
                                    dir: global_transform.compute_transform()
                                        .rotation,
                                    source: player_entity,
                                });

                                turret.0 = GunState::reload();
                            }
                        }
                    }
                });
        });
}

fn update_rotation<const DEBUG: bool>(
    transform: &mut Transform,
    rotate_angle: f32,

    rapier_context: &Res<RapierContext>,
    player: Entity,

    // gizmos: &mut Gizmos
) {
    let pos = transform.translation;
    let new_rot = {
        let dir = transform.up().as_vec3();
        
        get_rotation_z(Vec2::new(dir.x, dir.y)) + rotate_angle
    };
    
    if DEBUG {
        // gizmos.rect_2d(Vec2::new(pos.x, pos.y), new_rot, Vec2::splat(TANK_SIZE), GREEN);
    }

    let cast_shape = {
        let shape = Collider::cuboid(TANK_SIZE/2., TANK_SIZE/2.);
        let shape_pos = Vec2::new(pos.x, pos.y);
        let shape_rot = new_rot;
        let shape_vel = Vec2::ONE;
        let filter = {
            let mut filter = QueryFilter::default();

            filter.exclude_collider = Some(player);

            filter
        };
        let options = ShapeCastOptions {
            max_time_of_impact: 0.0,
            target_distance: 0.0,
            stop_at_penetration: true,
            compute_impact_geometry_on_penetration: true,
        };
        
        rapier_context.cast_shape(
            shape_pos,
            shape_rot,
            shape_vel,
            &shape,
            options,
            filter
        )
    };

    if let Some((_entity, _hit)) = cast_shape {
        return ;
    }

    transform.rotate_z(rotate_angle);
}

fn new_move_pos<const DEBUG: bool>(
    start_pos: Vec3,
    dir: Vec3,
    delta_time: f32,

    rapier_context: &Res<RapierContext>,
    player: Entity,

    // gizmos: &mut Gizmos
) -> Vec3{
    
    let new_pos = start_pos + TANK_MOVE_SPEED * dir * delta_time;
    let rot = get_rotation_z(Vec2::new(dir.x, dir.y));
    if DEBUG {
        // gizmos.rect_2d(Vec2::new(new_pos.x, new_pos.y), rot, Vec2::splat(TANK_SIZE), GREEN);
    }

    let cast_shape = {
        let shape = Collider::cuboid(TANK_SIZE/2., TANK_SIZE/2.);
        let shape_pos = Vec2::new(new_pos.x, new_pos.y);
        let shape_rot = rot;
        let shape_vel = Vec2::new(dir.x, dir.y).normalize();
        let filter = {
            let mut filter = QueryFilter::default();

            filter.exclude_collider = Some(player);

            filter
        };
        let options = ShapeCastOptions {
            max_time_of_impact: TANK_MOVE_SPEED * delta_time,
            target_distance: 0.0,
            stop_at_penetration: true,
            compute_impact_geometry_on_penetration: true,
        };
        
        rapier_context.cast_shape(
            shape_pos,
            shape_rot,
            shape_vel,
            &shape,
            options,
            filter
        )
    };

    if let Some((_entity, hit)) = cast_shape {
        return start_pos + dir * hit.time_of_impact
    }


    start_pos + TANK_MOVE_SPEED * dir * delta_time
}

pub fn get_rotation_z(dir: Vec2) -> f32 {
    let mag = (dir.x.powi(2) + dir.y.powi(2)).sqrt();
    let normalized_dir = Vec2 { x: dir.x / mag, y: dir.y / mag };

    let up = Vec2 { x: 0.0, y: 1.0 };

    let dot_product = up.x * normalized_dir.x + up.y * normalized_dir.y;

    let det = up.x * normalized_dir.y - up.y * normalized_dir.x;

    let angle = det.atan2(dot_product);

    if angle < 0.0 {
        angle + 2.0 * PI
    } else {
        angle
    }
}