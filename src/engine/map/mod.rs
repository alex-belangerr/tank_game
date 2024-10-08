use bevy::{asset::{Asset, AssetServer, Handle}, prelude::{Component, Res}, reflect::Reflect};
use serde::{Deserialize, Serialize};

pub mod map_loader;
pub type Coord = (usize, usize);

#[derive(Debug, Clone, Asset, Reflect, Deserialize, Serialize)]
pub struct Map{
    dim: (usize, usize),
    walls: Vec<Coord>,
    spawn_points: Vec<Coord>
}

#[derive(Debug, Clone, Copy, Component)]
struct Wall;

pub fn generate_map(asset_server: Res<AssetServer>){
    println!("Hello from generate map");
    let map: Handle<Map> = asset_server.load("maps/map_1.ron");

    // println!("{map:?}");
    // println!("{:#?}", maps.get(map.id()));

    // let img = bmp::open("assets/map_1.bmp").unwrap_or_else(|e| {
    //     panic!("Failed to open: {}", e);
    // });

    // println!("{img:?}")
}