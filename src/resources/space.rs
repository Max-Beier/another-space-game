use std::ops::Range;

use bevy::prelude::Resource;

#[derive(Resource, Clone)]
pub struct Space {
    // Starsystem
    pub planets_count: Range<usize>,
    pub planets_distance: Range<f32>,
    // Star
    pub star_radius: Range<f32>,
    pub star_surface_gravity: Range<f32>,
    // Planet
    pub planet_radius: Range<f32>,
    pub planet_surface_gravity: Range<f32>,
}

impl Default for Space {
    fn default() -> Self {
        Self {
            // Starsystem
            planets_count: 1..10,
            planets_distance: 50000.0..100000.0,
            // Star
            star_radius: 5000.0..10000.0,
            star_surface_gravity: 30.0..50.0,
            // Planet
            planet_radius: 100.0..4000.0,
            planet_surface_gravity: 4.0..20.0,
        }
    }
}
