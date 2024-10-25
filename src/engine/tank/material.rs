use bevy::{asset::{Asset, Handle}, color::LinearRgba, prelude::Image, reflect::TypePath, render::render_resource::{AsBindGroup, ShaderRef}, sprite::Material2d};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[bind_group_data(TankMaterialKey)]
pub struct TankMaterial {
    #[uniform(0)]
    pub(crate) primary_colour: LinearRgba,
    #[uniform(1)]
    pub(crate) secondary_colour: LinearRgba,

    #[texture(2)]
    pub(crate) colour_texture: Handle<Image>,
}

impl Material2d for TankMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tank.wgsl".into()
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct TankMaterialKey {
}

impl From<&TankMaterial> for TankMaterialKey {
    fn from(_material: &TankMaterial) -> Self {
        Self {
        }
    }
}