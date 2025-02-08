use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub jump: bool,
    pub dash: bool,
}

pub fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
) {
    let mut movement = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        movement.x += 1.0;
    }
    let jump = keyboard_input.just_pressed(KeyCode::Space)
        || keyboard_input.just_pressed(KeyCode::KeyW)
        || keyboard_input.just_pressed(KeyCode::ArrowUp);

    let dash = keyboard_input.just_pressed(KeyCode::ShiftLeft)
        || keyboard_input.just_pressed(KeyCode::ShiftRight);

    player_input.movement = movement;
    player_input.jump = jump;
    player_input.dash = dash;
}

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>()
            .add_systems(Update, player_input_system);
    }
}