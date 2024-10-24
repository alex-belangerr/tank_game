use bevy::{asset::Asset, color::LinearRgba, reflect::TypePath, render::{mesh::MeshVertexBufferLayoutRef, render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError}}, sprite::Material2d};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// #[bind_group_data(TankMaterialKey)]
pub struct TankMaterial {
    #[uniform(0)]
    pub(crate) colour: LinearRgba,
}

impl Material2d for TankMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tank.wgsl".into()
    }

    // fn specialize(
    //     _descriptor: &mut RenderPipelineDescriptor,
    //     _layout: &MeshVertexBufferLayoutRef,
    //     _key: Material2dKey<Self>,
    // ) -> Result<(), SpecializedMeshPipelineError> {
    //     Ok(())
    // }
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