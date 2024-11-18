use bevy_ecs::{component::Component, entity::Entity};
use nalgebra::Transform3;

/// General ----------------------------------------------------------------------------------------
#[derive(Component, Debug, Clone)]
pub struct Parent {
    pub entity: Entity,
}

#[derive(Component, Debug, Default, Clone)]
pub struct Children {
    pub children: Vec<Entity>,
}

/// 2D ----------------------------------------------------------------------------------------
#[derive(Component, Debug, Default, Clone)]
pub struct Transform2D {
    pub transform: Transform3<f32>,
}

#[derive(Component, Debug, Default, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Default, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// 3D ----------------------------------------------------------------------------------------
#[derive(Component, Debug, Default, Clone)]
pub struct Transform3D {
    pub transform: Transform3<f32>,
}
