use std::net::IpAddr;

use bevy::{app::Plugin, prelude::KeyCode};

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerController<const P_FLAG: u32> {
    Server{
        ip: IpAddr,
        port: u16
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
    pub fn wasd() -> PlayerController<P_FLAG> {
        PlayerController::Control{
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            rotate_left: KeyCode::KeyA,
            rotate_right: KeyCode::KeyD,
            spin_turret_left: KeyCode::KeyQ,
            spin_turret_right: KeyCode::KeyE,
            shoot: KeyCode::Space,
        }
    }
    pub fn arrow() -> PlayerController<P_FLAG>{
        PlayerController::Control{
            move_forward: KeyCode::ArrowUp,
            move_backward: KeyCode::ArrowDown,
            rotate_left: KeyCode::ArrowLeft,
            rotate_right: KeyCode::ArrowRight,
            spin_turret_left: KeyCode::KeyI,
            spin_turret_right: KeyCode::KeyO,
            shoot: KeyCode::KeyP,
        }
    }
}


impl<const P_FLAG: u32>  Plugin for PlayerController<P_FLAG> {
    fn build(&self, app: &mut bevy::prelude::App) {
        println!("ADDING PLAYER PLUGIN {P_FLAG}");
    }
}