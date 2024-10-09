use bevy::{
    prelude::*,
};
use player::PlayerControllerPlugin;

// use bevy::prelude::AssetPlugin;

pub mod args;
pub mod player;
pub mod render;
pub mod engine;

fn main() {
    let game_builder = args::get_args();

    let mut app = App::new();

    // todo!() add game plugin
    
    app.add_plugins(engine::EnginePlugin(game_builder.render));
        // .add_plugins(AssetPlugin::default())
        // .add_systems(Startup, map::generate_map);

    app.add_plugins(PlayerControllerPlugin(game_builder.player_1, game_builder.player_2));
    
    if game_builder.render {
        app.add_plugins(render::GameRenderPlugin);
    }
    
    app.run();
}