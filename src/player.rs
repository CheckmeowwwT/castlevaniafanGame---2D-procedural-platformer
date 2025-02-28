use bevy::prelude::*;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout, SpriteSheetBundle};
use crate::control::PlayerInput;
use crate::map::{Map, TileType, PLAYER_START_X, PLAYER_START_Y};

pub const PLAYER_SIZE: f32 = 1.0;
const HALF_PLAYER: f32 = PLAYER_SIZE / 2.0;
const PLAYER_SPEED: f32 = 6.0;
const JUMP_FORCE: f32 = 19.0;
const GRAVITY: f32 = 28.0;
const DASH_SPEED: f32 = 16.0;
const DASH_DURATION: f32 = 0.18;
const DASH_COOLDOWN: f32 = 0.28;
const ANIMATION_FPS: f32 = 10.0;
pub const PLAYER_SCALE: f32 = 3.0;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub is_grounded: bool,
    pub jump_request: f32,
    pub is_dashing: bool,
    pub dash_timer: f32,
    pub dash_cooldown: f32,
    pub facing: f32,
    pub anim_state: AnimationState,
    pub anim_timer: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            is_grounded: false,
            jump_request: 0.0,
            is_dashing: false,
            dash_timer: 0.0,
            dash_cooldown: 0.0,
            facing: 1.0,
            anim_state: AnimationState::Walk,
            anim_timer: 0.0,
        }
    }
}

#[derive(Component)]
pub struct CameraFollow;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Walk,
    Jump,
    Dash,
}

pub trait Jumpable {
    fn try_jump(&mut self);
}

pub trait Dashable {
    fn try_dash(&mut self, direction: f32);
}

pub trait Animatable {
    fn set_animation(&mut self, state: AnimationState);
    fn update_animation(&mut self, delta: f32, max_frame: usize) -> usize;
}

impl Jumpable for Player {
    fn try_jump(&mut self) {
        if self.is_grounded && self.jump_request > 0.0 {
            self.velocity.y = JUMP_FORCE;
            self.is_grounded = false;
            self.jump_request = 0.0;
            self.anim_state = AnimationState::Jump;
            self.anim_timer = 0.0;
        }
    }
}

impl Dashable for Player {
    fn try_dash(&mut self, direction: f32) {
        if !self.is_dashing && self.dash_cooldown <= 0.0 && direction != 0.0 {
            self.is_dashing = true;
            self.dash_timer = DASH_DURATION;
            self.dash_cooldown = DASH_COOLDOWN;
            self.velocity.x = direction * DASH_SPEED;
            self.anim_state = AnimationState::Dash;
            self.anim_timer = 0.0;
        }
    }
}

impl Animatable for Player {
    fn set_animation(&mut self, state: AnimationState) {
        if self.anim_state != state {
            self.anim_state = state;
            self.anim_timer = 0.0;
        }
    }
    fn update_animation(&mut self, delta: f32, max_frame: usize) -> usize {
        self.anim_timer += delta;
        let frame = ((self.anim_timer * ANIMATION_FPS) as usize) % max_frame;
        frame
    }
}

#[derive(Resource)]
pub struct PlayerSpriteHandles {
    pub atlas: Handle<TextureAtlasLayout>,
    pub img: Handle<Image>,
}

pub fn load_player_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let img: Handle<Image> = asset_server.load("player_sheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(84.0), 4, 1, None, None);
    let atlas = atlases.add(layout);

    commands.insert_resource(PlayerSpriteHandles { atlas, img });
}

pub fn player_movement_system(
    time: Res<Time>,
    map: Res<Map>,
    player_input: Res<PlayerInput>,
    mut q: Query<(&mut Player, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    if let Ok((mut player, mut transform)) = q.get_single_mut() {
        if player.is_dashing {
            player.dash_timer -= delta;
            if player.dash_timer <= 0.0 {
                player.is_dashing = false;
                player.velocity.x = 0.0;
            }
        } else {
            player.dash_cooldown -= delta;
        }
        if player_input.dash && !player.is_dashing && player.dash_cooldown <= 0.0 {
            let dir = if player_input.movement.x != 0.0 {
                player_input.movement.x.signum()
            } else {
                player.facing 
            };
            player.try_dash(dir);
        }
        if !player.is_dashing {
            player.velocity.x = player_input.movement.x * PLAYER_SPEED;
            if player_input.movement.x != 0.0 {
                player.facing = player_input.movement.x.signum();
            }
        }

        if !player.is_grounded && !player.is_dashing {
            if player.velocity.y < 0.0 {
                player.velocity.y -= GRAVITY * 1.7 * delta; 
            } else {
                player.velocity.y -= GRAVITY * delta;
            }
        }

        if player_input.jump {
            player.jump_request = 0.25;
        } else if player.jump_request > 0.0 {
            player.jump_request -= delta;
        }

        transform.translation.x += player.velocity.x * delta;

transform.translation.x += player.velocity.x * delta;

if player.velocity.x != 0.0 {
    let direction = player.velocity.x.signum() as i32;
    let check_x = (transform.translation.x + direction as f32 * HALF_PLAYER).floor() as i32;
    let feet_y = transform.translation.y - HALF_PLAYER;
    let head_y = transform.translation.y + HALF_PLAYER;
    let bottom_y = feet_y.floor() as i32;
    let top_y = head_y.floor() as i32;
    let mut horizontal_collision = false;
    for check_y in bottom_y..=top_y {
        if map.get_tile(check_x, check_y).is_collidable() {
            horizontal_collision = true;
            break;
        }
    }
    
    if horizontal_collision {
        transform.translation.x = if direction > 0 {
            check_x as f32 - HALF_PLAYER - 0.01
        } else {
            check_x as f32 + 1.0 + HALF_PLAYER + 0.01
        };
        player.velocity.x = 0.0;
    }
}
transform.translation.y += player.velocity.y * delta;
        transform.translation.y += player.velocity.y * delta;
        let feet_y = transform.translation.y - HALF_PLAYER;
        let tile_x = transform.translation.x.floor() as i32;
        let tile_y = (feet_y - 0.1).floor() as i32;
        if player.velocity.y <= 0.0 && map.get_tile(tile_x, tile_y).is_collidable() {
            transform.translation.y = (tile_y as f32 + 1.0) + HALF_PLAYER;
            player.velocity.y = 0.0;
            player.is_grounded = true;
        } else {
            player.is_grounded = false;
        }

        player.try_jump();

        if player.is_dashing {
            player.set_animation(AnimationState::Dash);
        } else if !player.is_grounded {
            player.set_animation(AnimationState::Jump);
        } else if player_input.movement.x != 0.0 {
            player.set_animation(AnimationState::Walk);
        } else {
            player.set_animation(AnimationState::Idle);
        }
    }
}

pub fn player_animation_system(
    time: Res<Time>,
    mut q: Query<(&mut Player, &mut TextureAtlas, &mut Sprite)>,
) {
    if let Ok((mut player, mut atlas, mut sprite)) = q.get_single_mut() {
        let delta = time.delta_seconds();
        
        sprite.flip_x = player.facing > 0.0;
        
        let index = match player.anim_state {
            AnimationState::Idle => 0,
            AnimationState::Walk => player.update_animation(delta, 4),
            AnimationState::Jump => 2, // not implemented yet
            AnimationState::Dash => 3, // or this 
        };
        atlas.index = index;
    }
}

pub fn spawn_player(
    mut commands: Commands,
    handles: Res<PlayerSpriteHandles>,
    map: Res<Map>,
) {
    let player_scale = match map.current_level {
        1 => PLAYER_SCALE,
        _ => PLAYER_SCALE * (64.0/84.0),
    };
    
    commands.spawn((
        Player::default(),
        SpriteBundle {
            texture: handles.img.clone(),
            transform: Transform::from_translation(Vec3::new(
                PLAYER_START_X as f32,
                PLAYER_START_Y as f32,
                1.0,
            ))
            .with_scale(Vec3::splat(player_scale)),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                flip_x: false,
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: handles.atlas.clone(),
            index: 0,
        }
    ));
}

pub fn camera_follow_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    map: Res<Map>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let max_camera_x = (map.level_length - 15) as f32;
            let target_x = player_transform.translation.x.min(max_camera_x);
            camera_transform.translation.x = target_x;
        }
    }
}pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_player_sprite)
           .add_systems(Startup, spawn_player.after(load_player_sprite))
           .add_systems(Update, (
               player_movement_system,
               player_animation_system,
               camera_follow_system,
           ));
    }
}