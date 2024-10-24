//! This module handles the gameplay mechanics for tanks in a Bevy-based game,
//! including their creation, movement, turret control, and associated instructions.

use bevy::{app::{Plugin, Update}, pbr::MaterialPlugin, sprite::Material2dPlugin};
use bullet::{bullet_collision, create_bullet, create_bullet_minimal, reload_gun, update_bullet_pos, NewBullet};
use instruction::{process_tank_instruction, Instruction};
use material::TankMaterial;
use vision::{update_tank_vision_ray, update_turret_vision_ray, NUM_OF_HULL_RAY, NUM_OF_TURRET_RAY};

pub mod instruction;
pub mod gen;
pub mod vision;
pub mod bullet;
pub mod material;

pub struct TankPlugin(pub bool);

impl Plugin for TankPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<Instruction<0>>()
            .add_event::<Instruction<1>>()
            .add_event::<NewBullet>()
            .add_systems(Update, process_tank_instruction::<0>)
            .add_systems(Update, process_tank_instruction::<1>)
            .add_systems(Update, update_tank_vision_ray::<NUM_OF_HULL_RAY>)
            .add_systems(Update, update_turret_vision_ray::<NUM_OF_TURRET_RAY>)
            .add_systems(Update, update_bullet_pos)
            .add_systems(Update, bullet_collision)
            .add_systems(Update, reload_gun);
        
        match self.0 {
            true => {
                app.add_plugins(Material2dPlugin::<TankMaterial>::default());
                app.add_systems(Update, create_bullet);
            },
            false => {
                app.add_systems(Update, create_bullet_minimal);
            }
        }
    }
}