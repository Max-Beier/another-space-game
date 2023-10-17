use bevy::prelude::{
    Bundle, ComputedVisibility, GlobalTransform, Handle, Mesh, StandardMaterial, Transform, Vec3,
    Visibility,
};

use crate::components::PName;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PName,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            name: Default::default(),
            mesh: Default::default(),
            material: Default::default(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 35.0),
                ..Default::default()
            },
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}
