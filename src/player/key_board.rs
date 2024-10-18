use bevy::{input::ButtonInput, prelude::{EventWriter, KeyCode, Res, Resource}};

use crate::engine::tank::instruction::Instruction;

use super::PlayerController;


/// Handles keyboard input for player controls and sends instructions based on key presses.
/// 
/// # Parameters
/// - `player_keybinding`: Resource containing key bindings for the player.
/// - `keys`: Resource containing the current state of key inputs.
/// - `event_writer`: Event writer for sending instructions based on input.
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
