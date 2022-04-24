mod unit_prefab;
mod weapon;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

pub use self::{unit_prefab::*, weapon::*};

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Component)]
pub struct Scores {
    pub amount: u32,
}
#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Component, Clone, Serialize, Deserialize)]
pub struct Loot {
    pub score: u32,
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(amount: u32) -> Self {
        Self {
            current: amount,
            max: amount,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}
