use bevy::prelude::{Component, Vec3};

use super::AtmosphereSettings;

#[derive(Component, Clone, Copy)]
pub enum CBClass {
    Planet,
    Star,
    _Moon,
    _Asteroid,
}

impl Default for CBClass {
    fn default() -> Self {
        Self::Star
    }
}

#[derive(Component, Clone)]
pub struct CBOrbit {
    pub center_origin: Vec3,
    pub radius: f32,
    pub velocity: f32,
}

impl Default for CBOrbit {
    fn default() -> Self {
        Self {
            center_origin: Vec3::ZERO,
            radius: 0.0,
            velocity: 0.0,
        }
    }
}

#[derive(Component, Clone)]
pub struct CelestialBody {
    pub name: String,
    pub class: CBClass,
    pub radius: f32,
    pub surface_gravity: f32,
    pub spin_velocity: f32,
    pub orbit: Option<CBOrbit>,
    pub atmosphere: AtmosphereSettings,
}

impl Default for CelestialBody {
    fn default() -> Self {
        Self {
            name: "Celestial Body".to_string(),
            class: Default::default(),
            radius: 6371008.767,
            surface_gravity: 9.81,
            spin_velocity: 464.0,
            orbit: None,
            atmosphere: AtmosphereSettings::default(),
        }
    }
}
