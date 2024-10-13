use bevy::{
    a11y::AccessibilityPlugin, app::{PanicHandlerPlugin, Plugin, Update}, asset::AssetPlugin, diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::{HierarchyPlugin, TransformPlugin}, state::app::StatesPlugin, DefaultPlugins, MinimalPlugins
};
use map::MapPlugin;
use tank::{process_tank_instruction, Instruction};
// use bevy_app::{Plugin, PluginGroup, PluginGroupBuilder};

pub mod map;
pub mod tank;

pub struct EnginePlugin(pub bool);

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        match self.0 {
            true => {
                app.add_plugins(DefaultPlugins);
            },
            false => {
                app.add_plugins(MinimalPlugins)
                    .add_plugins(PanicHandlerPlugin)
                    .add_plugins(LogPlugin::default())
                    .add_plugins(TransformPlugin)
                    .add_plugins(HierarchyPlugin)
                    .add_plugins(DiagnosticsPlugin)
                    .add_plugins(AccessibilityPlugin)
                    .add_plugins(AssetPlugin::default())
                    .add_plugins(StatesPlugin);
            },
        };
        app
            .add_plugins(MapPlugin(self.0))
            .add_event::<Instruction<0>>()
            .add_event::<Instruction<1>>()
            .add_systems(Update, process_tank_instruction::<0>)
            .add_systems(Update, process_tank_instruction::<1>);
        
    }
}