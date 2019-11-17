use amethyst::{
    assets::{Handle, Prefab},
    core::transform::Transform,
    ecs::prelude::*,
    prelude::Builder,
};

use crate::components::{
    Animation,
    AnimationId,
    AnimationPrefabData,
    Direction,
    Directions,
    Motion,
    Player,
    PLAYER_WIDTH,
};

use crate::states::{GAME_HEIGHT, GAME_WIDTH};

/// Initialises one player in the middle-ish space
pub fn load_player(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>) {
    let mut transform = Transform::default();

    // Correctly position the player in the middle for now.
    let y = GAME_HEIGHT / 2.0 + 20.0;
    transform.set_translation_xyz((GAME_WIDTH / 2.0) - PLAYER_WIDTH * 0.5, y, 0.0);

    let motion = Motion::new();

    // Create a player entity.
    world
        .create_entity()
        .with(Player::new())
        .with(transform)
        .with(Animation::new(
            AnimationId::Idle,
            vec![
                AnimationId::Die,
                AnimationId::Jump,
                AnimationId::Idle,
                AnimationId::Walk,
                AnimationId::Duck,
            ],
        ))
        .with(prefab)
        .with(motion)
        .with(Direction::new(
            Directions::Right,
            Directions::Neutral,
            Directions::Right,
            Directions::Neutral,
        ))
        .build();
}
