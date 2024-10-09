use bevy::{app::{Plugin, Startup, Update}, asset::{Asset, AssetApp, AssetServer, Assets, Handle}, prelude::{in_state, AppExtStates, Component, IntoSystemConfigs, NextState, Res, ResMut, Resource, State}, reflect::Reflect};
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

#[derive(Debug, Clone, Copy, Component)]
struct Wall;

pub fn load_map(asset_server: Res<AssetServer>, mut current_map: ResMut<CurrentMap>, mut next_state: ResMut<NextState<Step>>){
    let map: Handle<Map> = asset_server.load("maps/map_1.ron");

    let current_map = current_map.as_mut();
    current_map.0 = Some(map);

    next_state.set(Step::GenerateMap);
}

pub fn generate_map(current_map: Res<CurrentMap>, maps: Res<Assets<Map>>, mut next_state: ResMut<NextState<Step>>){
    let current_map = current_map.as_ref();
    let map_id = current_map.0.clone().unwrap().id();

    let Some(map) = maps.as_ref().get(map_id) else {
        return;
    };

    println!("{map:?}");
    // generate walls & pick spawn points
    next_state.set(Step::Finished);
}

pub struct MapPlugin;

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
            )
            .add_systems(
                Update,
                (
                    generate_map.run_if(in_state(Step::GenerateMap)),
                )
            );
    }
}