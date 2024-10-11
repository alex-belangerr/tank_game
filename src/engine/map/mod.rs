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

const WALL_SIZE: f32 = 1.;

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

    commands.spawn(Camera2dBundle::default());


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