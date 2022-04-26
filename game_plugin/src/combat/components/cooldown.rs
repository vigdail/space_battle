use std::time::Duration;

use bevy::prelude::*;
use serde::{de::Visitor, Deserialize, Serialize};

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Default, Component, Clone)]
pub struct Cooldown(#[cfg_attr(feature = "debug", inspectable(ignore))] pub Timer);

impl<'de> Deserialize<'de> for Cooldown {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CooldownVisitor;
        impl<'de> Visitor<'de> for CooldownVisitor {
            type Value = Cooldown;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Expected number")
            }

            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
                Ok(value.into())
            }
        }

        deserializer.deserialize_f32(CooldownVisitor)
    }
}

impl Serialize for Cooldown {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f32(self.0.duration().as_secs_f32())
    }
}

impl Cooldown {
    pub fn from_seconds(seconds: f32) -> Self {
        Self(Timer::new(Duration::from_secs_f32(seconds), false))
    }
}

impl From<f32> for Cooldown {
    fn from(secs: f32) -> Self {
        Self::from_seconds(secs)
    }
}
