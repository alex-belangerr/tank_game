use bevy::{
    a11y::AccessibilityPlugin, app::{PanicHandlerPlugin, Plugin}, asset::AssetPlugin, diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::{HierarchyPlugin, TransformPlugin}, state::app::StatesPlugin, DefaultPlugins, MinimalPlugins
};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

#[cfg(feature = "debug")]
use bevy_rapier2d::render::RapierDebugRenderPlugin;

#[cfg(feature = "cinematic")]
use camera::update_camera_pos;
#[cfg(feature = "cinematic")]
use bevy::app::Update;

use map::MapPlugin;
use tank::TankPlugin;

pub mod map;
pub mod tank;
mod camera;

pub struct EnginePlugin(pub bool);

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        match self.0 {
            true => {
                app.add_plugins(DefaultPlugins);
                    // .insert_resource(AmbientLight {
                    //     color: Color::WHITE,
                    //     brightness: 1.0 / 5.0f32,
                    // });
                
                #[cfg(feature = "debug")]
                app.add_plugins(RapierDebugRenderPlugin::default());

                #[cfg(feature = "cinematic")]
                app.add_systems(Update, update_camera_pos);
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
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(8.0))
            .add_plugins(MapPlugin(self.0))
            .add_plugins(TankPlugin(self.0));
    }
}