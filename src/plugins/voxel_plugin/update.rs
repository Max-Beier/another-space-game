use bevy::{
    ecs::{query::With, system::Query},
    transform::components::Transform,
};

use super::OctreeObject;
use crate::components::PlayerController;

pub fn update(
    q_player_position: Query<&Transform, With<PlayerController>>,
    mut q_octree_objects: Query<(&mut OctreeObject, &mut Transform)>,
) {
    let player_position = q_player_position.single();

    for (mut octree_object, octree_position) in q_octree_objects.iter_mut() {
        let distance = octree_position
            .translation
            .distance_squared(player_position.translation);
    }
}
