use bevy::prelude::*;

#[derive(Default)]
struct Speed {
    x: f32,
    y: f32,
}

#[derive(Default, Component)]
pub struct Bird {
    speed: Speed,
}
