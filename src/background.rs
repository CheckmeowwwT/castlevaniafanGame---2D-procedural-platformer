use bevy::prelude::*;
use noise::{Perlin, NoiseFn};
use crate::map::{MAP_WIDTH, MAP_HEIGHT, TILE_SIZE};
use crate::assets::LevelAssets;

#[derive(Component)]
pub struct BackgroundLayer;

pub fn spawn_background(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
) {
    let perlin = Perlin::new(1337);
    let width = MAP_WIDTH as i32;
    let height = MAP_HEIGHT as i32 * 2; 
    let scale = 0.15;
    let threshold = 0.15; 

    for (layer_idx, (z, col, alpha, x_offset, y_offset)) in [
        ( -10.0, Color::rgb(0.3, 0.5, 0.6), 0.6, 0.0, 0.0),
        ( -20.0, Color::rgb(0.2, 0.25, 0.4), 0.32, 30.0, -5.0 )
    ].iter().enumerate() {
        if !level_assets.background_textures.is_empty() && layer_idx < level_assets.background_textures.len() {
            commands.spawn((
                SpriteBundle {
                    texture: level_assets.background_textures[layer_idx].clone(),
                    transform: Transform::from_translation(Vec3::new(width as f32 / 2.0 * TILE_SIZE, 
                                                                    height as f32 / 2.0 * TILE_SIZE, 
                                                                    *z)),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(width as f32 * TILE_SIZE * 2.0, height as f32 * TILE_SIZE)),
                        ..default()
                    },
                    ..default()
                },
                BackgroundLayer,
            ));
        } else {
            for y in 0..height {
                for x in 0..width {
                    let nx = (x as f64 + x_offset) * scale;
                    let ny = (y as f64 + y_offset) * scale;
                    let val = perlin.get([nx, ny]);
                    if val > threshold {
                        commands.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: col.with_a(*alpha),
                                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                                    ..default()
                                },
                                transform: Transform::from_translation(
                                    Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, *z)
                                ),
                                ..default()
                            },
                            BackgroundLayer,
                        ));
                    }
                }
            }
        }
    }
}