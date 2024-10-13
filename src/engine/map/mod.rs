use bevy::{
    app::{Plugin, Startup, Update
    },
    asset::{Asset, AssetApp, AssetServer, Assets, Handle},
    math::Vec3,
    prelude::{
        in_state, AppExtStates, Camera2dBundle, Commands, Component, GlobalTransform, Image, InheritedVisibility, IntoSystemConfigs, NextState, Res, ResMut, Resource, Transform, ViewVisibility, Visibility
    },
    reflect::Reflect,
    sprite::Sprite
};
use gen_state::Step;
use map_loader::MapLoader;
use serde::{Deserialize, Serialize};

use crate::{engine::tank::{create_minimal_tank, create_tank}, player::PlayerID};

pub mod map_loader;
pub mod gen_state;
pub type Coord = (usize, usize);

#[derive(Debug, Clone, Asset, Reflect, Deserialize, Serialize)]
pub struct Map{
    dim: (usize, usize),
    walls: Vec<Coord>,
    spawn_points: Vec<Coord>
}

#[derive(Debug, Clone, Resource, Default)]
pub struct CurrentMap(pub Option<Handle<Map>>);

const WALL_SIZE: f32 = 32.;

#[derive(Debug, Clone, Copy, Component)]
struct Wall;

pub fn load_map(asset_server: Res<AssetServer>, mut current_map: ResMut<CurrentMap>, mut next_state: ResMut<NextState<Step>>){
    let map: Handle<Map> = asset_server.load("maps/map_1.ron");

    let current_map = current_map.as_mut();
    current_map.0 = Some(map);

    next_state.set(Step::GenerateMap);
}

pub fn generate_minimal_map(
    mut commands: Commands,
    
    current_map: Res<CurrentMap>,
    maps: Res<Assets<Map>>,
    
    mut next_state: ResMut<NextState<Step>>
){
    let current_map = current_map.as_ref();
    let map_id = current_map.0.clone().unwrap().id();

    let Some(map) = maps.as_ref().get(map_id) else {
        return;
    };

    println!("{map:?}");
    // generate walls & pick spawn points
    type WallBundle = (Wall,Transform,GlobalTransform);
    let walls: Vec<WallBundle> = map.walls.iter()
        .map(|(x, y)| (*x as f32, *y as f32))
        .map( |(x, y)| (
            Wall,
            Transform{
                translation: Vec3{
                    x: x * WALL_SIZE,
                    y: y * WALL_SIZE,
                    z: 0.,
                },
                ..Default::default()
            },
            Default::default()
        ))
        .collect();
    commands.spawn_batch(
        walls
    );

    let p1_spawn = {
        let i1 = rand::random::<usize>() % map.spawn_points.len();

        &map.spawn_points[i1]
    };

    let p2_spawn = map.spawn_points
        .iter()
        .filter(|&&point| point != *p1_spawn)
        .fold(
            (*p1_spawn, 0isize),
            |acc, next| {
                let dist = (next.0 as isize - p1_spawn.0 as isize).pow(2) + (next.1 as isize - p1_spawn.1 as isize).pow(2);
                
                match dist > acc.1 {
                    true => (*next, dist),
                    false => acc
                }
            }
        ).0;

    {
        let p1 = create_minimal_tank(
            p1_spawn.0 as f32 * WALL_SIZE,
            p1_spawn.1 as f32 * WALL_SIZE,
            0,
            &mut commands
        );
        commands.entity(p1)
            .insert(PlayerID::<0>);
    }

    {
        let p2 = create_minimal_tank(
            p2_spawn.0 as f32 * WALL_SIZE,
            p2_spawn.1 as f32 * WALL_SIZE,
            1,
            &mut commands
        );
        commands.entity(p2)
            .insert(PlayerID::<1>);
    }
    
    next_state.set(Step::Finished);
}

pub fn generate_map(
    mut commands: Commands,
    
    current_map: Res<CurrentMap>,
    maps: Res<Assets<Map>>,

    asset_server: Res<AssetServer>,
    
    mut next_state: ResMut<NextState<Step>>
){
    let current_map = current_map.as_ref();
    let map_id = current_map.0.clone().unwrap().id();

    let Some(map) = maps.as_ref().get(map_id) else {
        return;
    };

    // println!("{map:?}");
    // generate walls & pick spawn points
    type WallBundle = (
        Wall,
        Transform,
        GlobalTransform,

        Sprite,
        Handle<Image>,
        Visibility,
        InheritedVisibility,
        ViewVisibility,
    );
    let walls: Vec<WallBundle> = map.walls.iter()
        .map(|(x, y)| (*x as f32, *y as f32))
        .map( |(x, y)| (
            Wall,
            Transform{
                translation: Vec3{
                    x: x * WALL_SIZE,
                    y: y * WALL_SIZE,
                    z: 0.,
                },
                ..Default::default()
            },
            Default::default(),
            
            Default::default(),
            asset_server.load("textures\\map\\wall.png"),
            Default::default(),
            Default::default(),
            Default::default()
        ))
        .collect();
    commands.spawn_batch(
        walls
    );

    commands.spawn((
        Camera2dBundle{
            transform: Transform{
                translation: Vec3 {
                    x: map.dim.0 as f32 / 2. * WALL_SIZE,
                    y: map.dim.1 as f32 / 2. * WALL_SIZE,
                    z: 1. 
                },
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    
    let p1_spawn = {
        let i1 = rand::random::<usize>() % map.spawn_points.len();

        &map.spawn_points[i1]
    };

    let p2_spawn = map.spawn_points
        .iter()
        .filter(|&&point| point != *p1_spawn)
        .fold(
            (*p1_spawn, 0isize),
            |acc, next| {
                let dist = (next.0 as isize - p1_spawn.0 as isize).pow(2) + (next.1 as isize - p1_spawn.1 as isize).pow(2);
                
                match dist > acc.1 {
                    true => (*next, dist),
                    false => acc
                }
            }
        ).0;
    {
        let p1 = create_tank(
            p1_spawn.0 as f32 * WALL_SIZE,
            p1_spawn.1 as f32 * WALL_SIZE,
            0,
            &mut commands,
            &asset_server
        );
        commands.entity(p1)
            .insert(PlayerID::<0>);
    }

    {
        let p2 = create_tank(
            p2_spawn.0 as f32 * WALL_SIZE,
            p2_spawn.1 as f32 * WALL_SIZE,
            1,
            &mut commands,
            &asset_server
        );
        commands.entity(p2)
            .insert(PlayerID::<1>);
    }


    next_state.set(Step::Finished);
}

pub struct MapPlugin(pub bool);

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_asset::<Map>()
            .init_asset_loader::<MapLoader>()

            .init_resource::<CurrentMap>()

            .init_state::<Step>()
            .add_systems(
                Startup,
                load_map.run_if(in_state(Step::LoadMap))
            );
        match self.0 {
            false => {
                app.add_systems(
                    Update,
                    (
                        generate_minimal_map.run_if(in_state(Step::GenerateMap)),
                    )
                );
            },
            true => {
                app.add_systems(
                    Update,
                    (
                        generate_map.run_if(in_state(Step::GenerateMap)),
                    )
                );
            }
        }
    }
}