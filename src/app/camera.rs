use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::transform::GlobalTransform;
use amethyst::ecs::prelude::Entity;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection};

use app::consts;

pub fn create_camera(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            consts::ARENA_WIDTH,
            consts::ARENA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
        ))
        .build()
}
