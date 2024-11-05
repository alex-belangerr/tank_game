//! This module provides functionality for player control in the game.
//! It includes player input handling, player control definitions, 
//! and the configuration for controlling tanks.

use std::net::IpAddr;

use bevy::{app::{Plugin, Update}, input::InputPlugin, prelude::{in_state, Component, IntoSystemConfigs, KeyCode}};
use key_board::{keyboard_input, PlayerKeyBind};
use server::{end_game_msg, server_input, update_player_data, PlayerServer};

use crate::engine::map::gen_state::Step;

pub mod server;
pub mod key_board;

/// Represents a unique identifier for a player.
#[derive(Component)]
pub struct PlayerID<const P_FLAG: u32>;

/// A plugin that manages player controls for two players.
pub struct PlayerControllerPlugin<const P_FLAG_1: u32, const P_FLAG_2: u32>(
    pub PlayerController<P_FLAG_1>,
    pub PlayerController<P_FLAG_2>
);

impl<const P_FLAG_1: u32, const P_FLAG_2: u32> Plugin for PlayerControllerPlugin<P_FLAG_1, P_FLAG_2>{
    fn build(&self, app: &mut bevy::prelude::App) {
        match (&self.0, &self.1, app.is_plugin_added::<InputPlugin>()) {
            (_, PlayerController::Control{ .. }, false) |
            (PlayerController::Control { .. }, _, false) => {
                app.add_plugins(InputPlugin);
            },
            _ => {}
        }
        
        match (&self.0, &self.1) {
            (_, PlayerController::Server{ .. }) |
            (PlayerController::Server { .. }, _) => {
                // sever initialization code
                // app.init_resource::<GameServer>();
            },
            _ => {}
        }

        // Copy values over
        app.add_plugins(self.0.clone())
            .add_plugins(self.1.clone());
    }
}

/// Represents the type of control a player can have.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PlayerController<const P_FLAG: u32> {
    Server{
        ip: IpAddr,
        port: u16,
        game_id: String,
    },
    Control{
        move_forward: KeyCode,
        move_backward: KeyCode,
        rotate_left: KeyCode,
        rotate_right: KeyCode,

        spin_turret_left: KeyCode,
        spin_turret_right: KeyCode,
        shoot: KeyCode
    }
}
impl<const P_FLAG: u32> PlayerController<P_FLAG> {
    
    /// Creates a `PlayerController` configured for WASD controls.
    pub fn wasd() -> PlayerController<P_FLAG> {
        PlayerController::Control{
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            rotate_left: KeyCode::KeyD,
            rotate_right: KeyCode::KeyA,
            spin_turret_left: KeyCode::KeyE,
            spin_turret_right: KeyCode::KeyQ,
            shoot: KeyCode::Space,
        }
    }

    /// Creates a `PlayerController` configured for arrow key controls.
    pub fn arrow() -> PlayerController<P_FLAG>{
        PlayerController::Control{
            move_forward: KeyCode::ArrowUp,
            move_backward: KeyCode::ArrowDown,
            rotate_left: KeyCode::ArrowRight,
            rotate_right: KeyCode::ArrowLeft,
            spin_turret_left: KeyCode::KeyO,
            spin_turret_right: KeyCode::KeyI,
            shoot: KeyCode::KeyP,
        }
    }
}

impl<const P_FLAG: u32> Plugin for PlayerController<P_FLAG> {
    fn build(&self, app: &mut bevy::prelude::App) {
        println!("ADDING PLAYER PLUGIN {P_FLAG}");
        match &self {
            PlayerController::Server { ip, port, game_id  } => { // todo!() replace placeholder with a higher order function that interacts with server
                app.insert_resource::<PlayerServer<P_FLAG>>(PlayerServer::new(*ip, *port, &game_id))
                    .add_systems(Update, server_input::<P_FLAG>)
                    .add_systems(Update, update_player_data::<P_FLAG>)
                    .add_systems(
                        Update,
                        end_game_msg::<P_FLAG>.run_if(in_state(Step::Finished))
                    );
            },
            PlayerController::Control { .. } => { // todo!() replace placeholder with a higher order function that creates keyboard_input using key mapping
                app.insert_resource::<PlayerKeyBind<P_FLAG>>(self.into())
                    .add_systems(Update, keyboard_input::<P_FLAG>);
                println!("key board controls");
            },
        }
    }
}