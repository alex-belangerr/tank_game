use bevy::{
    a11y::AccessibilityPlugin, app::{PanicHandlerPlugin, Plugin}, asset::AssetPlugin, diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::{HierarchyPlugin, TransformPlugin}, state::app::StatesPlugin, DefaultPlugins, MinimalPlugins
};
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use map::MapPlugin;
use tank::TankPlugin;

pub mod map;
pub mod tank;

pub struct EnginePlugin(pub bool);

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        match self.0 {
            true => {
                app.add_plugins(DefaultPlugins)
                    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(8.0))
                    //DEBUG
                    .add_plugins(RapierDebugRenderPlugin::default());
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
            .add_plugins(TankPlugin);        
    }
}