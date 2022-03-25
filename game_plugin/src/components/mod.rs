use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component)]
pub struct Player;

#[derive(Component, Inspectable)]
pub struct Velocity(pub Vec3);
