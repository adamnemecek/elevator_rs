use amethyst::{
    core::math::{Vector2, Vector3},
    ecs::{Component, DenseVecStorage, NullStorage},
};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Rideable;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct ElevatorComponent {
    pub name: &'static str,
    pub sprite_number: usize,
    pub width: f32,
    pub height: f32,
    pub offsets: Vector3<f32>,
    pub is_collidable: bool,
}

impl ElevatorComponent {
    pub fn new(
        name: &'static str,
        sprite_number: usize,
        width: f32,
        height: f32,
        offsets: Vector3<f32>,
        is_collidable: bool,
    ) -> Self {
        ElevatorComponent {
            name,
            sprite_number,
            width,
            height,
            offsets,
            is_collidable,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum ElevatorState {
    Up,
    Down,
    Waiting,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Elevator {
    pub position: Vector2<f32>,
    pub boundaries: Vec<f32>,
    pub floor_height: f32,
    pub num_floors: usize,
    pub start_floor: usize,
    pub current_floor: f32,
    pub velocity: f32,
    pub previous_state: ElevatorState,
    pub state: ElevatorState,
    pub wait_seconds: f64,
    pub can_wait: bool,
}

impl Default for Elevator {
    fn default() -> Self {
        Elevator {
            position: Vector2::new(0., 0.),
            boundaries: Vec::new(),
            floor_height: 48.,
            num_floors: 1,
            start_floor: 0,
            current_floor: 0.,
            velocity: 0.,
            previous_state: ElevatorState::Waiting,
            state: ElevatorState::Waiting,
            wait_seconds: 0.,
            can_wait: true,
        }
    }
}

impl Elevator {
    pub fn new(
        position: Vector2<f32>,
        num_floors: usize,
        start_floor: usize,
        current_floor: f32,
        velocity: f32,
    ) -> Self {
        let mut boundaries: Vec<f32> = Vec::new();
        let floored_current_floor: usize = current_floor.floor() as usize;
        for i in 1..=num_floors {
            boundaries.push(
                position.y - ((floored_current_floor - start_floor) as f32 * 48.)
                    + (i - 1) as f32 * 48.,
            );
        }
        println!("Set elevator boundaries -- boundaries: {:?}", boundaries);
        Elevator {
            position,
            boundaries,
            num_floors,
            start_floor,
            current_floor,
            velocity,
            ..Elevator::default()
        }
    }
}