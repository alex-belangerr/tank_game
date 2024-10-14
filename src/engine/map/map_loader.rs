//! This module defines the `MapLoader` for loading map assets in RON format.
//! It handles the parsing and reading of map files, providing error handling
//! for any issues that may arise during the process.
use std::{error::Error, fmt::{self, Display}, future::Future};
use bevy::{asset::{AssetLoader, AsyncReadExt}, utils::ConditionalSendFuture};
use ron::de::SpannedError;

use super::Map;

/// An enumeration of errors that may occur during the loading of map assets.
/// 
/// # Variants
/// - `ParsingError`: Indicates an error occurred while parsing the RON data.
/// - `ReadingError`: Indicates an error occurred while reading the file.
#[derive(Default)]
pub struct MapLoader;

#[derive(Debug)]
pub enum MapLoaderError{
    ParsingError(SpannedError),
    ReadingError(std::io::Error)
}
impl Display for MapLoaderError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapLoaderError::ParsingError(_spanned_error) => {
                todo!()
                // write!(f, format!("{:#?}", spanned_error))
            },
            MapLoaderError::ReadingError(_io_error) => {
                todo!()
                // write!(f, format!("{:#?}", io_error))
            },
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
        MapLoaderError::ParsingError(value)
    }
}
impl From<std::io::Error> for MapLoaderError{
    fn from(value: std::io::Error) -> Self {
        MapLoaderError::ReadingError(value)
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
