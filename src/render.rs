use bevy::{a11y::AccessibilityPlugin, app::{PanicHandlerPlugin, Plugin, Startup}, asset::Assets, color::Color, core_pipeline::CorePipelinePlugin, diagnostic::DiagnosticsPlugin, gizmos::GizmoPlugin, log::LogPlugin, math::Vec2, prelude::{Annulus, Camera2dBundle, Capsule2d, Circle, CircularSector, CircularSegment, Commands, Ellipse, ImagePlugin, Mesh, Rectangle, RegularPolygon, ResMut, Rhombus, TextBundle, Transform, Triangle2d}, render::{pipelined_rendering::PipelinedRenderingPlugin, texture, RenderPlugin}, scene::ScenePlugin, sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle, SpritePlugin}, text::{TextPlugin, TextStyle}, ui::{PositionType, Style, UiPlugin, Val}, window::WindowPlugin, DefaultPlugins};
use bevy::prelude::default;


const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shapes = [
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(CircularSector::new(50.0, 1.0))),
        Mesh2dHandle(meshes.add(CircularSegment::new(50.0, 1.25))),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Annulus::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rhombus::new(75.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn(
        TextBundle::from_section("Press space to toggle wireframes", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );
}

pub struct GameRenderPlugin;

impl Plugin for GameRenderPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        // app
        //     .add_plugins(SpritePlugin)
        //     .add_plugins(TextPlugin)
        //     .add_plugins(UiPlugin)
        //     .add_plugins(GizmoPlugin)
        //     .add_plugins(ScenePlugin)
        //     .add_plugins(RenderPlugin::default()) // The core render plugin
        //     .add_plugins(ImagePlugin::default()) // For texture support
        //     .add_plugins(PipelinedRenderingPlugin) // For pipelined rendering
        //     .add_plugins(CorePipelinePlugin) // For core pipeline rendering (2D/3D)
        //     .add_plugins(WindowPlugin::default());
            // .add_plugins(DefaultPlugins);
            // .add_plugins(RenderPlugin)
            // .add_plugins(texture::ImagePlugin::default())
            // .add_plugins(WindowPlugin::default());
        // todo!() - Add rendering stuff
        //  - add custom material
        //  - Add camera stuff
        //  - Maybe add a start timer
        // println!("Render plugin")

        app.add_systems(Startup, setup);
    }
}