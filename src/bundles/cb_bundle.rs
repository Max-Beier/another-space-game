use bevy::prelude::Bundle;

use crate::components::{CBClass, CBName, CBRadius, CBSpin};

#[derive(Bundle)]
pub struct CBBundle {
    pub name: CBName,
    pub class: CBClass,
    pub radius: CBRadius,
    pub spin: CBSpin,
}

impl Default for CBBundle {
    fn default() -> Self {
        Self {
            name: Default::default(),
            class: Default::default(),
            radius: Default::default(),
            spin: Default::default(),
        }
    }
}
