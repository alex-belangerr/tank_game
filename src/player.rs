use std::net::IpAddr;

use bevy::{app::{Plugin, Update}, input::{ButtonInput, InputPlugin}, prelude::{Component, EventWriter, KeyCode, Res, Resource}};

use crate::engine::tank::Instruction;

#[derive(Component)]
pub struct PlayerID<const P_FLAG: u32>;

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
            rotate_left: KeyCode::KeyD,
            rotate_right: KeyCode::KeyA,
            spin_turret_left: KeyCode::KeyE,
            spin_turret_right: KeyCode::KeyQ,
            shoot: KeyCode::Space,
        }
    }
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

#[derive(Resource)]
pub struct PlayerKeyBind<const P_FLAG: u32>{
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub rotate_left: KeyCode,
    pub rotate_right: KeyCode,

    pub spin_turret_left: KeyCode,
    pub spin_turret_right: KeyCode,
    pub shoot: KeyCode
}

impl<const P_FLAG: u32> From<&PlayerController<P_FLAG>> for PlayerKeyBind<P_FLAG> {
    fn from(value: &PlayerController<P_FLAG>) -> Self {
        let PlayerController::Control{
            move_forward,
            move_backward,
            rotate_left,
            rotate_right,
        
            spin_turret_left,
            spin_turret_right,
            shoot
        } = *value else {
            panic!("Invalid call");
        };

        PlayerKeyBind{
            move_forward,
            move_backward,
            rotate_left,
            rotate_right,
            spin_turret_left,
            spin_turret_right,
            shoot,
        }
    }
}

pub fn keyboard_input<const P_FLAG: u32>(
    player_keybinding: Res<PlayerKeyBind<P_FLAG>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<Instruction<P_FLAG>>
){
    if keys.pressed(player_keybinding.move_forward) {
        event_writer.send(Instruction::MoveForward);
    }
    else if keys.pressed(player_keybinding.move_backward) {
        event_writer.send(Instruction::MoveBackward);
    }

    if keys.pressed(player_keybinding.rotate_left) {
        event_writer.send(Instruction::RotateLeft);
    }
    else if keys.pressed(player_keybinding.rotate_right) {
        event_writer.send(Instruction::RotateRight);
    }

    if keys.pressed(player_keybinding.spin_turret_left) {
        event_writer.send(Instruction::SpinTurretLeft);
    }
    else if keys.pressed(player_keybinding.spin_turret_right) {
        event_writer.send(Instruction::SpinTurretRight);
    }

    if keys.pressed(player_keybinding.shoot) {
        event_writer.send(Instruction::Shoot);
    }

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
                app.insert_resource::<PlayerKeyBind<P_FLAG>>(self.into());
                app.add_systems(Update, keyboard_input::<P_FLAG>);
                println!("key board controls");
            },
        }
    }
}