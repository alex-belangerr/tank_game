use std::{error::Error, fmt::{self, Display}, future::Future};
use bevy::{asset::{AssetLoader, AsyncReadExt}, utils::ConditionalSendFuture};
use ron::de::SpannedError;

use super::Map;


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
