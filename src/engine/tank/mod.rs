//! This module handles the gameplay mechanics for tanks in a Bevy-based game,
//! including their creation, movement, turret control, and associated instructions.

use bevy::app::{Plugin, Update};
use instruction::{process_tank_instruction, Instruction};
use vision::{update_tank_vision_ray, update_turret_vision_ray, NUM_OF_HULL_RAY, NUM_OF_TURRET_RAY};

pub mod instruction;
pub mod gen;
pub mod vision;


pub struct TankPlugin;

impl Plugin for TankPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<Instruction<0>>()
            .add_event::<Instruction<1>>()
            .add_systems(Update, process_tank_instruction::<0>)
            .add_systems(Update, process_tank_instruction::<1>)
            .add_systems(Update, update_tank_vision_ray::<NUM_OF_HULL_RAY, false>)
            .add_systems(Update, update_turret_vision_ray::<NUM_OF_TURRET_RAY, false>);
        }
}