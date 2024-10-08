use std::{error::Error, fmt::{self, Display}, future::Future, io::Cursor};
use bevy::{asset::{Asset, AssetLoader, AssetServer, AsyncReadExt, Handle}, prelude::{default, Component, Image, Res}, reflect::{Reflect, TypePath}, utils::{BoxedFuture, ConditionalSendFuture}};
use ron::de::SpannedError;
use serde::{Deserialize, Serialize};

type Coord = (usize, usize);

const MAX_MAP_WIDTH: usize = 32;
const MAX_MAP_HEIGHT: usize = 32;

#[derive(Default)]
pub struct MapLoader;

#[derive(Debug, Clone, Copy)]
pub enum MapLoaderError{
    FileDoesNotExist
}
impl Display for MapLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapLoaderError::FileDoesNotExist => {
                write!(f, "The file does not exist.")
            }
        }
    }
}
impl Error for MapLoaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<SpannedError> for MapLoaderError{
    fn from(value: SpannedError) -> Self {
        todo!()
    }
}
impl From<std::io::Error> for MapLoaderError{
    fn from(value: std::io::Error) -> Self {
        todo!()
    }
}

impl AssetLoader for MapLoader{
    type Asset = Map;

    type Settings = ();

    type Error = MapLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> impl ConditionalSendFuture + Future<Output = Result<<Self as AssetLoader>::Asset, <Self as AssetLoader>::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let custom_asset = ron::de::from_bytes::<Map>(&bytes)?;

            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

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

    println!("{map:?}")
    // let img = bmp::open("assets/map_1.bmp").unwrap_or_else(|e| {
    //     panic!("Failed to open: {}", e);
    // });

    // println!("{img:?}")
}