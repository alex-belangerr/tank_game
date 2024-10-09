use bevy::{a11y::AccessibilityPlugin, app::{PanicHandlerPlugin, Plugin}, diagnostic::DiagnosticsPlugin, log::LogPlugin, window::WindowPlugin};

pub struct RenderPlugin;

impl Plugin for RenderPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(WindowPlugin::default());
        // todo!() - Add rendering stuff
        //  - add custom material
        //  - Add camera stuff
        //  - Maybe add a start timer
        // println!("Render plugin")
    }
}