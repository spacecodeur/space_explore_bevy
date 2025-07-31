use bevy_common::{Component, Vec2, Timer, TimerMode};
use std::time::Duration;

#[derive(Component)]
pub enum MovementState {
    Idle,
    WalkLeft,
    WalkRight,
    // For diagonal movements
    //   WalkUpLeft,
    //   WalkUpRight,
    //   WalkDownLeft,
    //   WalkDownRight,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
    }
}
