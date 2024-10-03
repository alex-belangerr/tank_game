use bevy::{
    prelude::*,
};

pub mod args;
pub mod player;
pub mod render;

fn main() {
    let game_builder = args::get_args();

    let mut app = App::new();

    // todo!() add game plugin

    app.add_plugins(game_builder.player_1);
    app.add_plugins(game_builder.player_2);

    if game_builder.render {
        app.add_plugins(render::RenderPlugin);
    }
    
    app.run();
}