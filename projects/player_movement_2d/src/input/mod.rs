use bevy_common::{App, ButtonInput, Event, EventWriter, KeyCode, Plugin, Res, Update, Vec2};

pub struct InputPlugin;

#[derive(Event)]
pub struct InputEvent {
    pub direction: Vec2,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>()
            .add_systems(Update, handle_keyboard_input);
    }
}

fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut movement_events: EventWriter<InputEvent>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    // Normalize to prevent diagonal movement from being faster
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    // Send event even if direction == Vec2::ZERO (to handle stopping)
    movement_events.write(InputEvent { direction });
}
