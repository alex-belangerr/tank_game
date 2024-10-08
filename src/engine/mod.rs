use bevy::{a11y::AccessibilityPlugin, app::{PanicHandlerPlugin, Plugin, Startup}, asset::{AssetApp, AssetPlugin}, core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin}, diagnostic::DiagnosticsPlugin, input::InputPlugin, log::LogPlugin, prelude::{HierarchyPlugin, TransformPlugin}, time::TimePlugin, DefaultPlugins};
use map::{map_loader::MapLoader, Map, MapPlugin};
// use bevy_app::{Plugin, PluginGroup, PluginGroupBuilder};

pub mod map;

pub struct EnginePlugin;


impl Plugin for EnginePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(PanicHandlerPlugin)
            .add_plugins(LogPlugin::default())
            .add_plugins(TaskPoolPlugin::default())
            .add_plugins(TypeRegistrationPlugin)
            .add_plugins(FrameCountPlugin)
            .add_plugins(TimePlugin)
            .add_plugins(TransformPlugin)
            .add_plugins(HierarchyPlugin)
            .add_plugins(DiagnosticsPlugin)
            .add_plugins(InputPlugin)
//WindowPlugin
            .add_plugins(AccessibilityPlugin)
            .add_plugins(AssetPlugin::default())
            // .add_plugins(ScenePlugin::default())
            // .add_plugins(ScenePlugin)
//  - with feature bevy_asset
//  - with feature bevy_scene
// WinitPlugin - with feature bevy_winit
// RenderPlugin - with feature bevy_render
// ImagePlugin - with feature bevy_render
// PipelinedRenderingPlugin - with feature bevy_render when not targeting wasm32
// CorePipelinePlugin - with feature bevy_core_pipeline
// SpritePlugin - with feature bevy_sprite
// TextPlugin - with feature bevy_text
// UiPlugin - with feature bevy_ui
// PbrPlugin - with feature bevy_pbr
// GltfPlugin - with feature bevy_gltf
// AudioPlugin - with feature bevy_audio
// GilrsPlugin - with feature bevy_gilrs
// AnimationPlugin - with feature bevy_animation
// GizmoPlugin - with feature bevy_gizmos
// StatesPlugin - with feature bevy_state
// DevToolsPlugin - with feature bevy_dev_tools
// CiTestingPlugin - with feature bevy_ci_testing
            // .add_plugins(DefaultPlugins)
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
            .add_plugins(MapPlugin);
        
    }
}