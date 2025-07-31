use bevy_common::{App, EventReader, Plugin, Query, Res, Time, Transform, Update, With};

mod components;
pub use components::{Player, Velocity};

mod display;
use display::Display;

use crate::input::InputEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Display)
            .add_systems(Update, (apply_movement_input, apply_velocity));
    }
}

fn apply_movement_input(
    mut movement_events: EventReader<InputEvent>,
    mut player_query: Query<&mut Velocity, With<Player>>,
) {
    for event in movement_events.read() {
        for mut velocity in &mut player_query {
            let speed = 200.0; // pixels per second
            velocity.0 = event.direction * speed;
        }
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    for (mut transform, velocity) in &mut player_query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}
