//! This module defines the `Step` enum, which represents the various states
//! in the game or application flow. It is used to manage transitions 
//! between different stages of the game, particularly in the context of 
//! loading and generating maps.

use bevy::prelude::States;

/// Represents the different states in the application flow.
///
/// This enum is used to manage the sequence of operations related to map
/// loading and generation. Each variant corresponds to a specific stage
/// in the application's lifecycle.
///
/// # Variants
/// - `LoadMap`: The initial state where the application is loading a map asset.
/// - `GenerateMap`: The state where the application is generating the map from the loaded asset.
/// - `Finished`: The state indicating that the map generation is complete and the game is ready to proceed.
#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Step {
    #[default]
    LoadMap = 0,   // Initial state for loading the map
    GenerateMap = 1, // State for generating the map from the loaded asset
    Finished = 2,   // Final state indicating completion of map generation
}
