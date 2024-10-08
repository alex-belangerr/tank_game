use bevy::{
    a11y::AccessibilityPlugin, app::{PanicHandlerPlugin, Plugin, Startup}, asset::{AssetApp, AssetPlugin}, core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin}, diagnostic::DiagnosticsPlugin, input::InputPlugin, log::LogPlugin, prelude::{HierarchyPlugin, TransformPlugin}, scene::ScenePlugin, state::app::StatesPlugin, time::TimePlugin, window::WindowPlugin, DefaultPlugins, MinimalPlugins
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
            .add_plugins(InputPlugin)
            // .add_plugins(WindowPlugin::default())
            .add_plugins(AccessibilityPlugin)
            .add_plugins(AssetPlugin::default())
            .add_plugins(StatesPlugin)

            .add_plugins(MapPlugin);
        
    }
}