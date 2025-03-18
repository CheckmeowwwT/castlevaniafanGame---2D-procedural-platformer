use bevy::prelude::*;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout};
use crate::player::Player;

const TREE_SCALE: f32 = 1.5; // Increase scale
const ANIMATION_FPS: f32 = 8.0;
const PARALLAX_FACTOR: f32 = 0.4;
const TREE_ACTIVATION_X: f32 = 50.0; // Lower activation threshold for testing

#[derive(Component)]
pub struct TreesBackground {
    pub animation_timer: f32,
    pub active: bool,
    pub activation_threshold: f32,
}

#[derive(Resource)]
pub struct TreesSpriteHandles {
    pub atlas: Handle<TextureAtlasLayout>,
    pub img: Handle<Image>,
}

pub fn load_trees_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Loading trees sprite");
    let img: Handle<Image> = asset_server.load("background/trees.png");
    // Adjust dimensions based on your actual spritesheet
    let layout = TextureAtlasLayout::from_grid(Vec2::new(256.0, 256.0), 4, 1, None, None);
    let atlas = atlases.add(layout);

    commands.insert_resource(TreesSpriteHandles { atlas, img });
}

pub fn spawn_trees_background(
    mut commands: Commands,
    handles: Option<Res<TreesSpriteHandles>>,
) {
    if let Some(handles) = handles {
        info!("Spawning trees background");
        commands.spawn((
            SpriteBundle {
                texture: handles.img.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 10.0, -5.0))
                    .with_scale(Vec3::splat(TREE_SCALE)),
                visibility: Visibility::Hidden,
                ..default()
            },
            TextureAtlas {
                layout: handles.atlas.clone(),
                index: 0,
            },
            TreesBackground {
                animation_timer: 0.0,
                active: false,
                activation_threshold: TREE_ACTIVATION_X,
            },
        ));
    } else {
        error!("TreesSpriteHandles resource not found!");
    }
}

pub fn trees_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut TreesBackground, &mut TextureAtlas)>,
) {
    for (mut trees, mut atlas) in query.iter_mut() {
        if trees.active {
            trees.animation_timer += time.delta_seconds();
            let frame = ((trees.animation_timer * ANIMATION_FPS) as usize) % 4; // 4 frames
            atlas.index = frame;
        }
    }
}

pub fn trees_activation_system(
    player_query: Query<&Transform, With<Player>>,
    mut trees_query: Query<(&mut TreesBackground, &mut Transform, &mut Visibility), Without<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok((mut trees, mut transform, mut visibility)) = trees_query.get_single_mut() {
            // Debug log
            if !trees.active && player_transform.translation.x > trees.activation_threshold {
                info!("Activating trees at player position X: {}", player_transform.translation.x);
            }
            
            // Activate trees when player moves past threshold
            if player_transform.translation.x > trees.activation_threshold && !trees.active {
                trees.active = true;
                *visibility = Visibility::Visible;
                info!("Trees activated!");
            }
            
            if trees.active {
                // Update position for parallax effect
                transform.translation.x = player_transform.translation.x * PARALLAX_FACTOR;
            }
        } else {
            // Debug if query fails
            // info!("Trees query failed!");
        }
    }
}

pub struct TreesPlugin;

impl Plugin for TreesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_trees_sprite)
           .add_systems(Startup, spawn_trees_background.after(load_trees_sprite))
           .add_systems(Update, (
               trees_animation_system,
               trees_activation_system.after(crate::player::player_movement_system),
           ));
    }
}