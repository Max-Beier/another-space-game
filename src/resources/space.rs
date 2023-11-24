use std::ops::Range;

use bevy::prelude::Resource;

#[derive(Resource, Clone)]
pub struct Space {
    // General
    // ...
    // Celestial Body
    pub subdivisions: usize,
    // Starsystem
    pub planets_count: Range<usize>,
    pub planets_distance: Range<f32>,
    // Star
    pub star_radius: Range<f32>,
    pub star_surface_gravity: Range<f32>,
    pub star_atmosphere_radius: Range<f32>,
    // Planet
    pub planet_radius: Range<f32>,
    pub planet_surface_gravity: Range<f32>,
    pub planet_atmosphere_radius: Range<f32>,
    // Planet Orbit
    pub planet_orbit_velocity: Range<f32>,
}

impl Default for Space {
    fn default() -> Self {
        Self {
            // Celestial Body
            subdivisions: 20,
            // Starsystem
            planets_count: 1..10,
            planets_distance: 1000000.0..10000000.0,
            // Star
            star_radius: 5000.0..100000.0,
            star_surface_gravity: 1000.0..5000.0,
            star_atmosphere_radius: 1000.0..3000.0,
            // Planet
            planet_radius: 1000.0..4000.0,
            planet_surface_gravity: 5.0..1000.0,
            planet_atmosphere_radius: 1000.0..3000.0,
            // Planet Orbit
            planet_orbit_velocity: 0.00000000001..0.0000000001,
        }
    }
}
