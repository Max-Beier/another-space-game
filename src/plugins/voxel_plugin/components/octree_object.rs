use bevy::ecs::component::Component;

#[derive(Component)]
pub struct OctreeObject {
    pub size: f32,
    pub level: u8,
}
