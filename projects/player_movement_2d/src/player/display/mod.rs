use crate::player::components::{AnimationConfig, MovementState, Player, Velocity};
use bevy_common::{
    App, AssetServer, Assets, Commands, Handle, Image, IntoScheduleConfigs, Plugin, Query, Res, ResMut, 
    Sprite, Startup, TextureAtlas, TextureAtlasLayout, Time, UVec2, Update, Vec2, With,
};

pub struct Display;

impl Plugin for Display {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (update_movement_state, update_animation_state, execute_animations).chain());
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the combined sprite sheet with all animations
    let texture: Handle<Image> = asset_server.load("player.png");

    // Create texture atlas layout for combined animations (17 frames total)
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(100, 64), // Size of each frame
        17,                  // 17 frames total (idle + walking + others)
        1,                   // 1 row
        None,                // No padding
        None,                // No offset
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Create idle animation config (frames 0-3)
    let idle_animation = AnimationConfig::new(0, 3, 5);

    // Spawn player entity with idle animation
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: idle_animation.first_sprite_index,
            }),
            ..Default::default()
        },
        Player,
        Velocity(Vec2::ZERO),
        MovementState::Idle,
        idle_animation,
    ));
}

fn update_movement_state(
    mut query: Query<(&mut MovementState, &Velocity), With<Player>>,
) {
    for (mut movement_state, velocity) in &mut query {
        *movement_state = if velocity.0.length() < 0.1 {
            MovementState::Idle
        } else if velocity.0.x > 0.0 {
            MovementState::WalkRight
        } else if velocity.0.x < 0.0 {
            MovementState::WalkLeft
        } else {
            MovementState::Idle
        };
    }
}

fn update_animation_state(
    mut query: Query<(&mut AnimationConfig, &MovementState), With<Player>>,
) {
    for (mut config, movement_state) in &mut query {
        // Update animation config based on movement state
        let (first, last, fps) = match movement_state {
            MovementState::Idle => (0, 3, 5),       // Idle: frames 0-3, 5 FPS
            MovementState::WalkRight => (4, 10, 8), // Walking right: frames 4-10, 8 FPS
            MovementState::WalkLeft => (4, 10, 8),  // Walking left: frames 4-10, 8 FPS (we'll flip sprite later)
        };
        
        // Only update if the animation changed
        if config.first_sprite_index != first || config.last_sprite_index != last || config.fps != fps {
            config.first_sprite_index = first;
            config.last_sprite_index = last;
            config.fps = fps;
            config.frame_timer = AnimationConfig::timer_from_fps(fps);
        }
    }
}

// This system follows the official Bevy sprite animation example
fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<Player>>,
) {
    for (mut config, mut sprite) in &mut query {
        // Track how long the current sprite has been displayed
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    // ...and it IS the last frame, then we move back to the first frame
                    atlas.index = config.first_sprite_index;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame
                    atlas.index += 1;
                }
            }
        }
    }
}
