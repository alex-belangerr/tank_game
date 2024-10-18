use std::{f32::consts::PI, marker::PhantomData};

use bevy::{math::Vec2, prelude::{Component, Entity, GlobalTransform, Query, Res, With}};
use bevy_rapier2d::{na::{Matrix2, Vector2}, plugin::RapierContext, prelude::QueryFilter};
use serde::Serialize;

use crate::engine::map::Wall;

use super::gen::{Tank, Turret, TANK_SIZE};


pub const NUM_OF_HULL_RAY: usize = 8;
pub(super) const HULL_RAY_MAX_DIST: f32 = TANK_SIZE * 4.;

pub(super) const TURRET_VISION_ANGLE: f32 = PI / 12.;
pub const NUM_OF_TURRET_RAY: usize = 5;
pub(super) const TURRET_RAY_MAX_DIST: f32 = TANK_SIZE * 10.;

#[derive(Debug, Clone, Copy, Serialize)]
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

    // mut gizmos: Gizmos
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
                    // gizmos.line_2d(ray_pos, ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * max_toi, GREEN);
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
                                    // let hit_point = ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * toi;
                                    // gizmos.circle_2d(hit_point, 5., RED);
                                }
                                
                                Some(VisionHit::Enemy(toi))
                            },
                            (false, true) => {
                                if DEBUG {
                                    // let hit_point = ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * toi;
                                    // gizmos.circle_2d(hit_point, 5., BLUE);
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

    // mut gizmos: Gizmos
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
                    // gizmos.line_2d(ray_pos, ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * max_toi, GREEN);
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
                                    // let hit_point = ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * toi;
                                    // gizmos.circle_2d(hit_point, 5., RED);
                                }
                                
                                Some(VisionHit::Enemy(toi))
                            },
                            (false, true) => {
                                if DEBUG {
                                    // let hit_point = ray_pos + Vec2::new(ray_dir[0], ray_dir[1]) * toi;
                                    // gizmos.circle_2d(hit_point, 5., BLUE);
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
