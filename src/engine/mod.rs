use bevy::{
    a11y::AccessibilityPlugin, app::{PanicHandlerPlugin, Plugin, PreUpdate, Update}, asset::AssetPlugin, diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::{HierarchyPlugin, TransformPlugin}, state::app::StatesPlugin, DefaultPlugins, MinimalPlugins
};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

#[cfg(feature = "debug")]
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use camera::resize_camera;
#[cfg(feature = "cinematic")]
use camera::update_camera_pos;
#[cfg(feature = "cinematic")]
use camera::cinematic_camera_scale;

use game_time::{update_delta_time, DeltaTime};
use map::MapPlugin;
use tank::TankPlugin;

pub mod map;
pub mod tank;
mod camera;
mod game_time;

pub struct EnginePlugin(pub bool, pub Option<String>, pub Option<f32>);

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
                {
                    app.add_systems(Update, update_camera_pos)
                        .add_systems(Update, cinematic_camera_scale);
                }
                
                #[cfg(not(feature = "cinematic"))]
                app.add_systems(Update, resize_camera);
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

        match &self.2 {
            Some(delta_time) => {
                app.insert_resource(DeltaTime(*delta_time));
            },
            None => {
                app.insert_resource(DeltaTime(0.0))
                    .add_systems(PreUpdate, update_delta_time);
            },
        }
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(8.0))
            .add_plugins(MapPlugin(self.0, self.1.clone()))
            .add_plugins(TankPlugin(self.0));
    }
}