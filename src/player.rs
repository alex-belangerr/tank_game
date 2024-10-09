use std::net::IpAddr;

use bevy::{app::{Plugin, Update}, input::InputPlugin, prelude::KeyCode};

pub struct PlayerControllerPlugin<const P_FLAG_1: u32, const P_FLAG_2: u32>(
    pub PlayerController<P_FLAG_1>,
    pub PlayerController<P_FLAG_2>
);

impl<const P_FLAG_1: u32, const P_FLAG_2: u32> Plugin for PlayerControllerPlugin<P_FLAG_1, P_FLAG_2>{
    fn build(&self, app: &mut bevy::prelude::App) {
        match (&self.0, &self.1) {
            (_, PlayerController::Control{ .. }) |
            (PlayerController::Control { .. }, _) => {
                app.add_plugins(InputPlugin);
            },
            _ => {}
        }
        match (&self.0, &self.1) {
            (_, PlayerController::Server{ .. }) |
            (PlayerController::Server { .. }, _) => {
                // sever initialization code
            },
            _ => {}
        }

        // Copy values over
        app.add_plugins(self.0)
            .add_plugins(self.1);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

pub fn keyboard_input<const P_FLAG: u32>() {

}

impl<const P_FLAG: u32> Plugin for PlayerController<P_FLAG> {
    fn build(&self, app: &mut bevy::prelude::App) {
        println!("ADDING PLAYER PLUGIN {P_FLAG}");
        match &self {
            PlayerController::Server { .. } => { // todo!() replace placeholder with a higher order function that interacts with server
                println!("Server based control");
                // create thread to handle server inputs
                // initialize other server with our data
                //  - tank info
                //  - our server info (ip & port)
            },
            PlayerController::Control { .. } => { // todo!() replace placeholder with a higher order function that creates keyboard_input using key mapping
                // app.add_systems(Update, keyboard_input::<P_FLAG>);
                println!("key board controls");
            },
        }
    }
}