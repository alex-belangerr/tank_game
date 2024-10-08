use bevy::{app::{Plugin, Startup}, asset::{AssetApp, AssetPlugin}, DefaultPlugins};
// use bevy_app::{Plugin, PluginGroup, PluginGroupBuilder};

pub mod map;

pub struct EnginePlugin;


impl Plugin for EnginePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(DefaultPlugins)
            //.add_plugins(AssetPlugin::default())
            // .add_plugins(bevy_app::PanicHandlerPlugin)
            // .add_plugins(bevy_log::LogPlugin::default())
            // .add_plugins(bevy_core::TaskPoolPlugin::default())
            // .add_plugins(bevy_core::TypeRegistrationPlugin)
            // .add_plugins(bevy_core::FrameCountPlugin)
            // .add_plugins(bevy_time::TimePlugin)
            // .add_plugins(bevy_transform::TransformPlugin)
            // .add_plugins(bevy_hierarchy::HierarchyPlugin)
            // .add_plugins(bevy_diagnostic::DiagnosticsPlugin)
            // .add_plugins(bevy_input::InputPlugin)
            // .add_plugins(bevy_window::WindowPlugin::default())
            // .add_plugins(bevy_a11y::AccessibilityPlugin)

            .init_asset::<map::Map>()
            .init_asset_loader::<map::MapLoader>();
        
        app.add_systems(Startup, map::generate_map);
    }
}