use amethyst::{
    core::math::Vector2,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{Direction, Player, PlayerState, Motion};
use crate::resources::Context;

const GRAVITY_AMOUNT: f32 = -1.0;

pub struct PlayerKinematicsSystem;

impl<'s> System<'s> for PlayerKinematicsSystem {
    type SystemData = (
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Motion>,
        Read<'s, Context>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (dirs, players, mut motions, context) = data;

        for (dir, player, motion) in
            (&dirs, &players, &mut motions).join()
        {
            let mut acceleration = Vector2::new(0., 0.);
            match player.state {
                PlayerState::Idling | PlayerState::Ducking => {
                    // how much skidding happens
                    let acceleration_x = if motion.velocity.x != 0. { context.friction_amount } else { 0. };
                    acceleration = Vector2::new(acceleration_x, GRAVITY_AMOUNT);
                }
                PlayerState::Walking => {
                    acceleration = Vector2::new(context.walk_acceleration, GRAVITY_AMOUNT);
                }
                PlayerState::Jumping => {
                    // if collider.on_ground {
                    //     motion.velocity.y = player.max_jump_velocity;
                    //     collider.on_ground = false;
                    // }
                    // how much he slows down when he's in the air and not running
                    let acceleration_x = if motion.velocity.x != 0. { (-context.walk_acceleration/20.) } else { 0. };
                    acceleration = Vector2::new(acceleration_x, 0.);
                }
                // PlayerState::Dying => {
                //     if collider.on_ground {
                //         motion.velocity.x = 0.;
                //         motion.velocity.y = 8.;
                //         collider.on_ground = false;
                //     }
                //     acceleration = Vector2::new(0., GRAVITY_AMOUNT);
                // }
                _ => {}
            }
            motion.update_velocity(acceleration, dir, 0., player.max_ground_speed);
        }
    }
}
