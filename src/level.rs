use bevy::prelude::*;
use crate::player::Player;
use crate::map::Map;
use crate::assets::LevelAssets;

const TRANSITION_DURATION: f32 = 1.0;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Level1,
    Transitioning,
    Level2,
    Level3,
}

#[derive(Event)]
pub struct LevelChangeEvent {
    pub next_level: GameState,
}

#[derive(Resource, Default)]
pub struct LevelTransition {
    pub timer: f32,
    pub next_level: GameState,
    pub map_regenerated: bool,
}

#[derive(Component)]
pub struct FadeOverlay;

pub fn check_level_completion(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut transition: ResMut<LevelTransition>,
    map: Res<Map>,
) {
    if *game_state == GameState::Transitioning {
        return;
    }
    
    if let Ok(player_transform) = player_query.get_single() {
        let max_camera_x = (map.level_length - 15) as f32;
        if player_transform.translation.x > max_camera_x + 15.0 {
            let next_level = match *game_state.get() {
                GameState::Level1 => GameState::Level2,
                GameState::Level2 => GameState::Level3,
                _ => GameState::Level1,
            };
            
            transition.timer = TRANSITION_DURATION;
            transition.next_level = next_level;
            transition.map_regenerated = false;
            next_state.set(GameState::Transitioning);
            
            commands.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    z_index: ZIndex::Global(10),
                    ..default()
                },
                FadeOverlay,
            ));
        }
    }
}

pub fn update_transition_fade(
    time: Res<Time>,
    mut transition: ResMut<LevelTransition>,
    mut overlay_query: Query<&mut BackgroundColor, With<FadeOverlay>>,
    mut level_change_events: EventWriter<LevelChangeEvent>, 
) {
    transition.timer -= time.delta_seconds();
    
    if let Ok(mut background) = overlay_query.get_single_mut() {
        let alpha = if transition.timer > TRANSITION_DURATION / 2.0 {
            (1.0 - (transition.timer - TRANSITION_DURATION/2.0) / (TRANSITION_DURATION/2.0)).clamp(0.0, 1.0)
        } else {
            (transition.timer / (TRANSITION_DURATION/2.0)).clamp(0.0, 1.0)
        };
        
        background.0 = Color::rgba(0.0, 0.0, 0.0, alpha);
    }
    
    if transition.timer <= TRANSITION_DURATION / 2.0 && transition.timer > TRANSITION_DURATION / 2.0 - time.delta_seconds() && !transition.map_regenerated {
        level_change_events.send(LevelChangeEvent { next_level: transition.next_level.clone() });
        transition.map_regenerated = true;
    }
}

pub fn handle_level_change(
    mut commands: Commands,
    mut level_change_events: EventReader<LevelChangeEvent>,
    mut map: ResMut<Map>,
    asset_server: Res<AssetServer>, 
    mut level_assets: ResMut<LevelAssets>,
    map_query: Query<Entity, With<crate::map::MapRenderer>>,
    player_query: Query<Entity, With<crate::player::Player>>,
    level_seed: Res<crate::map::LevelSeed>,
) {
    for event in level_change_events.read() {
        let mut entities_removed = 0;
        for entity in map_query.iter() {
            commands.entity(entity).despawn_recursive();
            entities_removed += 1;
        }
        let next_level_num = match event.next_level {
            GameState::Level2 => 2,
            GameState::Level3 => 3,
            _ => 1,
        };
        
        level_assets.current_level = next_level_num;
        level_assets.block_textures.clear();
        
        let block_texture = match next_level_num {
            2 => asset_server.load("blocks/2.png"),
            3 => asset_server.load("blocks/3.png"),
            4 => asset_server.load("blocks/4.png"),
            _ => asset_server.load("blocks/1.png"),
        };
        
        level_assets.block_textures.push(block_texture);
        map.current_level = next_level_num;
        crate::map::generate_platformer_level(&mut map, &level_assets, &level_seed);

        if let Ok(player_entity) = player_query.get_single() {
            let player_scale = match next_level_num {
                1 => crate::player::PLAYER_SCALE,
                _ => crate::player::PLAYER_SCALE * (64.0/84.0),
            };
            
            commands.entity(player_entity).insert(Transform::from_translation(Vec3::new(
                crate::map::PLAYER_START_X as f32,
                crate::map::PLAYER_START_Y as f32,
                1.0,
            ))
            .with_scale(Vec3::splat(player_scale)));
            
        }
    }
}
pub fn finish_transition(
    mut commands: Commands,
    transition: Res<LevelTransition>,
    mut next_state: ResMut<NextState<GameState>>,
    overlay_entities: Query<Entity, With<FadeOverlay>>,
) {
    if transition.timer <= 0.0 {
        next_state.set(transition.next_level.clone());
        for entity in overlay_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
           .init_resource::<LevelTransition>()
           .add_event::<LevelChangeEvent>()
           .add_systems(Update, (
                check_level_completion,
                update_transition_fade.run_if(in_state(GameState::Transitioning)),
                handle_level_change,
                finish_transition.run_if(in_state(GameState::Transitioning)),
           ).chain());
    }
}