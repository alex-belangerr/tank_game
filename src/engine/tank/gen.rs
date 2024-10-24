use std::{f32::consts::PI, time::Duration};

use bevy::{asset::{AssetServer, Assets}, color::{palettes::css::PURPLE, Color, LinearRgba}, math::Vec3, pbr::MaterialMeshBundle, prelude::{default, BuildChildren, Commands, Component, Entity, GlobalTransform, Mesh, Rectangle, Res, ResMut, Transform}, render::{mesh::{Indices, PrimitiveTopology}, render_asset::RenderAssetUsages}, sprite::{ColorMaterial, MaterialMesh2dBundle, SpriteBundle}, time::Timer};
use bevy_rapier2d::prelude::Collider;

use super::{material::TankMaterial, vision::{VisionRay, HULL_RAY_MAX_DIST, NUM_OF_HULL_RAY, NUM_OF_TURRET_RAY, TURRET_RAY_MAX_DIST, TURRET_VISION_ANGLE}};



/// Represents a tank in the game.
///
/// # Fields
/// - `team_id`: The ID of the team that the tank belongs to.
/// - `turret`: The entity associated with the tank's turret.
#[derive(Component, Clone, Copy)]
pub struct Tank{
    pub team_id: u8,
    pub turret: Entity,
}

#[derive(Debug, Clone)]
pub enum GunState{
    Ready,
    Reload(Timer)
}

impl GunState {
    pub fn reload() -> Self {
        GunState::Reload(
            Timer::new(Duration::from_secs(1), bevy::time::TimerMode::Once)
        )
    }
}

impl Default for GunState {
    fn default() -> Self {
        GunState::Ready
    }
}

/// Represents a turret in the game.
///
/// This component is used to mark entities that act as turrets, which can rotate independently from the tank.
#[derive(Component, Debug)]
pub struct Turret(pub GunState);

impl Default for Turret{
    fn default() -> Self {
        Self(Default::default())
    }
}

pub const TANK_SIZE: f32 = 32.;

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
        Turret::default(),
        Transform{
            translation: Vec3{ x: 0., y: 0., z: TURRET_HEIGHT },
            ..Default::default()
        },
        GlobalTransform::default()
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

pub fn rect(scale: f32) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-1., -1., 0.0],
            [1., -1., 0.0],
            [1., 1.5, 0.0],
            [-1., 1., 0.0],
        ]
    );

    mesh.insert_indices(Indices::U32(vec![0, 1, 3, 3, 2, 1]));

    return mesh
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
pub fn create_tank(
    x: f32,
    y: f32,
    team_id: u8,
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<TankMaterial>>,
    asset_server: &Res<AssetServer>
) -> Entity {
    // todo!() - not worth it but could reduce repetition to run `create_minimal_tank` and just add sprite & texture on top
    let turret_id = commands.spawn((
        Turret::default(),
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
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_length(32.)).into(),
            transform: Transform{
                translation: Vec3{ x: x, y: y, z: TANK_HEIGHT },
                ..default()
            },
            material: materials.add(TankMaterial{ colour: LinearRgba::new(1., 0., 0., 1.)  }),
            ..default()
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

