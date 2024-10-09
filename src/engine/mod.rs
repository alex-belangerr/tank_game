use bevy::{
    a11y::AccessibilityPlugin, app::{PanicHandlerPlugin, Plugin}, asset::AssetPlugin, diagnostic::DiagnosticsPlugin, input::InputPlugin, log::LogPlugin, prelude::{HierarchyPlugin, TransformPlugin}, state::app::StatesPlugin, MinimalPlugins
};
use map::MapPlugin;
// use bevy_app::{Plugin, PluginGroup, PluginGroupBuilder};

pub mod map;

pub struct EnginePlugin;


impl Plugin for EnginePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(MinimalPlugins)
            .add_plugins(PanicHandlerPlugin)
            .add_plugins(LogPlugin::default())
            .add_plugins(TransformPlugin)
            .add_plugins(HierarchyPlugin)
            .add_plugins(DiagnosticsPlugin)
            .add_plugins(AccessibilityPlugin)
            .add_plugins(AssetPlugin::default())
            .add_plugins(StatesPlugin)

            .add_plugins(MapPlugin);
        
    }
}