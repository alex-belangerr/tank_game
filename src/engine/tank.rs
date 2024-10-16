//! This module handles the gameplay mechanics for tanks in a Bevy-based game,
//! including their creation, movement, turret control, and associated instructions.

use std::{f32::consts::PI, fmt::Debug, marker::PhantomData};

use bevy::{app::{Plugin, Update}, asset::AssetServer, color::palettes::css::{BLUE, GREEN, RED}, math::{Vec2, Vec3}, prelude::{BuildChildren, Commands, Component, Entity, Event, EventReader, Gizmos, GlobalTransform, Query, Res, Transform, With, Without}, sprite::SpriteBundle, time::Time};
use bevy_rapier2d::{na::{Matrix2, Vector2}, plugin::RapierContext, prelude::{Collider, QueryFilter}};

use crate::player::PlayerID;

use super::map::Wall;

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

const TANK_SIZE: f32 = 32.;

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
        GlobalTransform::default(),
        Collider::cuboid(TANK_SIZE/2., TANK_SIZE/2.),
        VisionRay::<NUM_OF_HULL_RAY, Tank>::new(
            HULL_RAY_MAX_DIST,
            2. * PI / NUM_OF_HULL_RAY as f32,
            0.
        ),
        VisionRay::<NUM_OF_TURRET_RAY, Turret>::new(
            TURRET_RAY_MAX_DIST,
            TURRET_VISION_ANGLE / NUM_OF_TURRET_RAY as f32,
            -TURRET_VISION_ANGLE / 2.
        )
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
        },
        Collider::cuboid(TANK_SIZE/2., TANK_SIZE/2.),
        VisionRay::<NUM_OF_HULL_RAY, Tank>::new(
            HULL_RAY_MAX_DIST,
            2. * PI / NUM_OF_HULL_RAY as f32,
            0.
        ),
        VisionRay::<NUM_OF_TURRET_RAY, Turret>::new(
            TURRET_RAY_MAX_DIST,
            TURRET_VISION_ANGLE / NUM_OF_TURRET_RAY as f32,
            -TURRET_VISION_ANGLE / 2.
        )
    )).id();

    commands.entity(tank_id).add_child(turret_id);

    tank_id
}

const NUM_OF_HULL_RAY: usize = 10;
const HULL_RAY_MAX_DIST: f32 = TANK_SIZE * 4.;

const TURRET_VISION_ANGLE: f32 = PI / 12.;
const NUM_OF_TURRET_RAY: usize = 5;
const TURRET_RAY_MAX_DIST: f32 = TANK_SIZE * 10.;

#[derive(Debug, Clone, Copy)]
pub enum VisionHit {
    Wall(f32),
    Enemy(f32)
}

#[derive(Component, Debug, Clone, Copy)]
pub struct VisionRay<const RAY_COUNT: usize, S> {
    pub rays: [Option<VisionHit>; RAY_COUNT],
    pub(self) max_dist: f32,
    pub(self) rotation_matrix: Matrix2<f32>,
    pub(self) start_dir: Matrix2<f32>,

    _phantom: PhantomData<S>
}
impl<const RAY_COUNT: usize, S> VisionRay<RAY_COUNT, S> {
    pub fn new(max_dist: f32, angle_gap: f32, start_angle: f32) -> Self{
        let rotation_matrix = Matrix2::new(
            f32::cos(angle_gap), -f32::sin(angle_gap),
            f32::sin(angle_gap), f32::cos(angle_gap)
        );

        let start_dir  = Matrix2::new(
            f32::cos(start_angle), -f32::sin(start_angle),
            f32::sin(start_angle), f32::cos(start_angle)
        );

        VisionRay {
            rays: [None; RAY_COUNT],
            max_dist,
            rotation_matrix,
            start_dir,

            _phantom: PhantomData,
        }
    }
}

/// Updates the vision rays for tanks, casting rays to detect walls and enemies.
/// 
/// # Parameters
/// 
/// * `rays`: A query containing mutable references to `VisionRay` components, 
///    global transformations, and associated entity.
/// * `tanks`: A query that filters entities with the `Tank` component.
/// * `walls`: A query that filters entities with the `Wall` component.
/// * `rapier_context`: A reference to the physics context (`RapierContext`) 
///    used for ray casting.
/// * `gizmos`: A mutable reference to `Gizmos` for visual debugging.
/// 
/// # Raycasting Logic
/// 
/// For each ray:
/// - Casts a ray in the direction specified by the tank's orientation.
/// - Checks for collisions with either walls or tanks.
/// - Marks the ray with the type of object hit (`Wall` or `Enemy`).
/// - Optionally displays debugging information such as the ray's path and 
///   hit points.
///
/// # Type Parameters
/// 
/// * `RAY_COUNT`: The number of rays to cast for vision.
/// * `DEBUG`: If true, displays debugging information for the rays.
/// 
/// # Panics
/// 
/// This function will panic if an unexpected collision type is detected, which 
/// should not happen given the query filters.
pub fn update_tank_vision_ray<const RAY_COUNT: usize, const DEBUG: bool>(
    mut rays: Query<(&mut VisionRay<RAY_COUNT, Tank>, &GlobalTransform, Entity)>,

    tanks: Query<(), With<Tank>>,
    walls: Query<(), With<Wall>>,

    rapier_context: Res<RapierContext>,

    mut gizmos: Gizmos
) {
    for (mut vision, transform, player_entity) in &mut rays {
        
        let vision = vision.as_mut();
        let VisionRay { rays, max_dist, rotation_matrix, start_dir, _phantom} = vision;

        let ray_pos = {
            let pos = transform.translation();

            Vec2::new(pos.x, pos.y)
        };

        let max_toi = *max_dist;
        let solid = true;
        let filter = {
            let mut filter = QueryFilter::default();

            filter.exclude_collider = Some(player_entity);

            filter
        };

        let mut ray_dir = {
            let forward = transform.up().as_vec3();
            let forward = *start_dir * Vector2::new(forward.x, forward.y);

            forward
        };

        rays.iter_mut()
            .for_each(|hit_marker| {
                if DEBUG {
                    gizmos.line_2d(ray_pos, ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * max_toi, GREEN);
                }
                
                let ray_cast = rapier_context.cast_ray(
                    ray_pos,
                    Vec2::new(ray_dir[0], ray_dir[1]),
                    max_toi,
                    solid,
                    filter
                );

                *hit_marker = match ray_cast {
                    Some((entity, toi)) => {
                        match (tanks.contains(entity), walls.contains(entity)) {
                            (true, false) => {
                                if DEBUG {
                                    let hit_point = ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * toi;
                                    gizmos.circle_2d(hit_point, 5., RED);
                                }
                                
                                Some(VisionHit::Enemy(toi))
                            },
                            (false, true) => {
                                if DEBUG {
                                    let hit_point = ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * toi;
                                    gizmos.circle_2d(hit_point, 5., BLUE);
                                }
                                
                                Some(VisionHit::Wall(toi))
                            },
                            _ => panic!("This should never happen")
                        }
                    },
                    None => None
                };

                ray_dir = *rotation_matrix * ray_dir;
            });
        if DEBUG {
            println!("{player_entity:?} - {rays:#?}")
        }
    }
}

/// Updates the vision rays for turrets, casting rays to detect walls and enemies.
///
/// # Parameters
///
/// * `rays`: A query containing mutable references to `VisionRay` components,
///    the associated tank, and entity.
/// * `turrets`: A query that filters entities with the `Turret` component to get
///    the global transform of the turret.
/// * `tanks`: A query that filters entities with the `Tank` component.
/// * `walls`: A query that filters entities with the `Wall` component.
/// * `rapier_context`: A reference to the physics context (`RapierContext`)
///    used for ray casting.
/// * `gizmos`: A mutable reference to `Gizmos` for visual debugging.
///
/// # Raycasting Logic
///
/// For each ray:
/// - Casts a ray in the direction specified by the turret's orientation.
/// - Checks for collisions with either walls or tanks.
/// - Marks the ray with the type of object hit (`Wall` or `Enemy`).
/// - Optionally displays debugging information such as the ray's path and
///   hit points.
///
/// # Type Parameters
///
/// * `RAY_COUNT`: The number of rays to cast for vision.
/// * `DEBUG`: If true, displays debugging information for the rays.
///
/// # Panics
///
/// This function will panic if the tank loses its reference to the turret or 
/// an unexpected collision type is detected.
pub fn update_turret_vision_ray<const RAY_COUNT: usize, const DEBUG: bool>(
    mut rays: Query<(&mut VisionRay<RAY_COUNT, Turret>, &Tank, Entity)>,

    turrets: Query<&GlobalTransform, With<Turret>>,
    tanks: Query<(), With<Tank>>,
    walls: Query<(), With<Wall>>,

    rapier_context: Res<RapierContext>,

    mut gizmos: Gizmos
) {
    for (mut vision, tank, player_entity) in &mut rays {

        let transform = turrets.get(tank.turret).expect("Tank lost ref to it's turret");
        
        let vision = vision.as_mut();
        let VisionRay { rays, max_dist, rotation_matrix, start_dir, _phantom} = vision;

        let ray_pos = {
            let pos = transform.translation();

            Vec2::new(pos.x, pos.y)
        };

        let max_toi = *max_dist;
        let solid = true;
        let filter = {
            let mut filter = QueryFilter::default();

            filter.exclude_collider = Some(player_entity);

            filter
        };

        let mut ray_dir = {
            let forward = transform.up().as_vec3();
            let forward = *start_dir * Vector2::new(forward.x, forward.y);

            forward
        };

        rays.iter_mut()
            .for_each(|hit_marker| {
                if DEBUG {
                    gizmos.line_2d(ray_pos, ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * max_toi, GREEN);
                }
                
                let ray_cast = rapier_context.cast_ray(
                    ray_pos,
                    Vec2::new(ray_dir[0], ray_dir[1]),
                    max_toi,
                    solid,
                    filter
                );

                *hit_marker = match ray_cast {
                    Some((entity, toi)) => {
                        match (tanks.contains(entity), walls.contains(entity)) {
                            (true, false) => {
                                if DEBUG {
                                    let hit_point = ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * toi;
                                    gizmos.circle_2d(hit_point, 5., RED);
                                }
                                
                                Some(VisionHit::Enemy(toi))
                            },
                            (false, true) => {
                                if DEBUG {
                                    let hit_point = ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * toi;
                                    gizmos.circle_2d(hit_point, 5., BLUE);
                                }
                                
                                Some(VisionHit::Wall(toi))
                            },
                            _ => panic!("This should never happen")
                        }
                    },
                    None => None
                };

                ray_dir = *rotation_matrix * ray_dir;
            });
        if DEBUG {
            println!("{player_entity:?} - {rays:#?}")
        }
    }
}

pub struct TankPlugin;

impl Plugin for TankPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<Instruction<0>>()
            .add_event::<Instruction<1>>()
            .add_systems(Update, process_tank_instruction::<0>)
            .add_systems(Update, process_tank_instruction::<1>)
            .add_systems(Update, update_tank_vision_ray::<NUM_OF_HULL_RAY, false>)
            .add_systems(Update, update_turret_vision_ray::<NUM_OF_TURRET_RAY, false>);
        }
}