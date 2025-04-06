use bevy::prelude::*;
use crate::player::Player;

const DEATH_HEIGHT: f32 = 0.0; 
const DEATH_TIME: f32 = 0.5;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameOverState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource, Default)]
pub struct FallTimer {
    pub time_below: f32,
}

pub fn check_player_death(
    player_query: Query<&Transform, With<Player>>,
    mut game_over_state: ResMut<NextState<GameOverState>>,
    current_state: Res<State<GameOverState>>,
    mut fall_timer: ResMut<FallTimer>,
    time: Res<Time>,
) {
    if *current_state == GameOverState::GameOver {
        return;
    }
    
    if let Ok(player_transform) = player_query.get_single() {
        if player_transform.translation.y < DEATH_HEIGHT {
            fall_timer.time_below += time.delta_seconds();
            
            if fall_timer.time_below >= DEATH_TIME {
                game_over_state.set(GameOverState::GameOver);
                fall_timer.time_below = 0.0;
            }
        } else {
            fall_timer.time_below = 0.0;
        }
    }
}

#[derive(Component)]
struct GameOverScreen;

pub fn display_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<GameOverScreen>>,
    state: Res<State<GameOverState>>,
) {
    if *state == GameOverState::GameOver && query.is_empty() {
        let game_over_texture = asset_server.load("game_over.png");
        
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                z_index: ZIndex::Global(100),
                ..default()
            },
            GameOverScreen,
        ))
        .with_children(|parent| {
            parent.spawn(
                ImageBundle {
                    image: UiImage::new(game_over_texture),
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(400.0),
                        ..default()
                    },
                    ..default()
                }
            );
        });
    }
}

pub fn restart_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_over_state: ResMut<NextState<GameOverState>>,
    state: Res<State<GameOverState>>,
    mut query: Query<Entity, With<GameOverScreen>>,
    mut commands: Commands,
) {
    if *state == GameOverState::GameOver && keys.just_pressed(KeyCode::KeyR) {
        for entity in query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
        game_over_state.set(GameOverState::Playing);
    }
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameOverState>()
           .init_resource::<FallTimer>() 
           .add_systems(Update, (
               check_player_death,
               display_game_over,
               restart_game,
           ));
    }
}