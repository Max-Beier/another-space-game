use std::ops::Range;

use bevy::prelude::Resource;

#[derive(Resource, Clone)]
pub struct Space {
    // Celestial Body
    pub subdivisions: usize,
    // Starsystem
    pub planets_count: Range<usize>,
    pub planets_distance: Range<f32>,
    // Star
    pub star_radius: Range<f32>,
    pub star_surface_gravity: Range<f32>,
    // Planet
    pub planet_radius: Range<f32>,
    pub planet_surface_gravity: Range<f32>,
    // Planet Orbit
    pub planet_orbit_velocity: Range<f32>,
}

impl Default for Space {
    fn default() -> Self {
        Self {
            // Celestial Body
            subdivisions: 10,
            // Starsystem
            planets_count: 1..10,
            planets_distance: 50000.0..100000.0,
            // Star
            star_radius: 5000.0..10000.0,
            star_surface_gravity: 50.0..500.0,
            // Planet
            planet_radius: 100.0..4000.0,
            planet_surface_gravity: 2.0..50.0,
            // Planet Orbit
            planet_orbit_velocity: 0.000001..0.00005,
        }
    }
}
