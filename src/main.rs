use bevy::prelude::*;
use player::PlayerControllerPlugin;

#[cfg(feature = "debug")]
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

pub mod args;
pub mod player;
pub mod engine;

fn main() {
    let game_builder = args::get_args();

    let mut app = App::new();

    app.add_plugins(engine::EnginePlugin(game_builder.render, game_builder.map.clone()));

    app.add_plugins(PlayerControllerPlugin(game_builder.player_1, game_builder.player_2));
    

    #[cfg(feature = "debug")]
    {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Update, |diagnostics: Res<DiagnosticsStore>,| println!("{:?}", diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.smoothed())));
    }

    app.run();
}