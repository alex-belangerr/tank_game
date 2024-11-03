//! This module handles the gameplay mechanics for tanks in a Bevy-based game,
//! including their creation, movement, turret control, and associated instructions.

use bevy::{app::{AppExit, Plugin, PostUpdate, Update}, prelude::{EventWriter, Query, With}, sprite::Material2dPlugin};
use bullet::{bullet_collision, create_bullet, create_bullet_minimal, reload_gun, update_bullet_pos, NewBullet};
use gen::Tank;
use instruction::{process_tank_instruction, Instruction};
use material::TankMaterial;
use vision::{update_tank_vision_ray, update_turret_vision_ray, NUM_OF_HULL_RAY, NUM_OF_TURRET_RAY};

use crate::player::PlayerID;

pub mod instruction;
pub mod gen;
pub mod vision;
pub mod bullet;
pub mod material;

fn end_game<const P_FLAG_1: u32, const P_FLAG_2: u32,>(
    tanks: Query<(Option<&PlayerID<P_FLAG_1>>, Option<&PlayerID<P_FLAG_2>>), With<Tank>>,

    mut app_exit_events: EventWriter<AppExit>,
) {
    let tank_count = tanks.iter()
        .map(|(tank_1, tank_2)| (
            tank_1.map(|_| 1u16).unwrap_or_else(|| 0u16),
            tank_2.map(|_| 1u16).unwrap_or_else(|| 0u16),
        ))
        .fold(
            (0u16, 0u16),
            |acc, next| (acc.0 + next.0, acc.1 + next.1)
        );

    match &tank_count {
        (0, 0) => {
            println!("-1")
        },
        (0, _) => {
            println!("{P_FLAG_2}")
        },
        (_, 0) => {
            println!("{P_FLAG_1}")
        },
        (_, _) => {
            return;
        }
    };

    app_exit_events.send(AppExit::Success);
}

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
            .add_systems(Update, reload_gun)
            .add_systems(PostUpdate, end_game::<0, 1>);
        
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