use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::level::GameState;

const LEVEL1_BG_COLOR: Color = Color::rgb(0.1, 0.1, 0.6);
const LEVEL2_BG_COLOR: Color = Color::rgb(0.5, 0.1, 0.1);
const LEVEL3_BG_COLOR: Color = Color::rgb(0.1, 0.3, 0.2); 

const SCALE: f64 = 0.15;
const THRESHOLD: f64 = 0.15;

#[derive(Component)]
pub struct BackgroundElement {
    pub level: u32,
    pub depth: f32,
}

#[derive(Component)]
pub struct ParallaxLayer {
    pub speed_factor: f32,
}

pub fn generate_level_background(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    query: Query<Entity, With<BackgroundElement>>,
    level_seed: Res<crate::map::LevelSeed>,
) {
    let level = match *game_state.get() {
        GameState::Level1 => 1,
        GameState::Level2 => 2,
        GameState::Level3 => 3,
        _ => 1,
    };
    
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    let clear_color = match level {
        2 => LEVEL2_BG_COLOR,
        3 => LEVEL3_BG_COLOR,
        _ => LEVEL1_BG_COLOR,
    };
    
    commands.insert_resource(ClearColor(clear_color));
    
    let seed = level_seed.seed + (level as u64 * 100);
    let perlin = Perlin::new(seed as u32);
    let mut rng = StdRng::seed_from_u64(seed);
    
    let background = commands.spawn(SpatialBundle::default()).id();
    commands.entity(background).insert(BackgroundElement {
        level,
        depth: -100.0,
    });
    
    generate_parallax_layer(
        &mut commands, 
        background, 
        &perlin, 
        &mut rng, 
        level, 
        -20.0,
        0.3,
        1.0, 
    );
    
    generate_parallax_layer(
        &mut commands,
        background,
        &perlin,
        &mut rng,
        level,
        -15.0,
        0.5,
        1.2, 
    );
    
    generate_parallax_layer(
        &mut commands,
        background,
        &perlin,
        &mut rng,
        level,
        -10.0,
        0.7, 
        1.5, 
    );
}

fn generate_parallax_layer(
    commands: &mut Commands,
    parent: Entity,
    perlin: &Perlin,
    rng: &mut StdRng,
    level: u32,
    z_layer: f32,
    parallax_factor: f32,
    density: f32,
) {
    let width = 300.0;
    let height = 100.0;
    let (col, alpha, threshold_mod) = match level {
        2 => (Color::rgb(0.6, 0.2, 0.1), 0.5, 0.05),  
        3 => (Color::rgb(0.05, 0.3, 0.1), 0.4, -0.02),   
        _ => (Color::rgb(0.05, 0.05, 0.4), 0.6, 0.0),    
    };
    let layer_entity = commands.spawn(SpatialBundle::default()).id();
    
    commands.entity(layer_entity).insert(BackgroundElement {
        level,
        depth: z_layer,
    });
    
    commands.entity(layer_entity).insert(ParallaxLayer {
        speed_factor: parallax_factor,
    });
    commands.entity(parent).add_child(layer_entity);
    
    for y in 0..(height as i32 * 2) {
        for x in 0..(width as i32) {
            let x_offset = rng.gen_range(-10.0..10.0);
            let y_offset = rng.gen_range(-5.0..5.0);
            
            let nx = (x as f64 + x_offset) * SCALE;
            let ny = (y as f64 + y_offset) * SCALE;
            
            let val = perlin.get([nx, ny]);
            
            if val > THRESHOLD + threshold_mod {
                let color_variance = rng.gen_range(-0.1..0.1);
                let mut adjusted_color = col.clone();
                
                match level {
                    2 => adjusted_color.set_r((col.r() + color_variance).clamp(0.0, 1.0)),
                    3 => adjusted_color.set_g((col.g() + color_variance).clamp(0.0, 1.0)),
                    _ => adjusted_color.set_b((col.b() + color_variance).clamp(0.0, 1.0)),
                };
                
                let size_variance = rng.gen_range(0.8..1.2) * density;
                let blob = commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: adjusted_color.with_a(alpha),
                        custom_size: Some(Vec2::splat(TILE_SIZE * size_variance)),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, z_layer - (y as f32 * 0.001))
                    ),
                    ..default()
                }).id();
                
                commands.entity(blob).insert(BackgroundElement {
                    level,
                    depth: z_layer,
                });
                
                commands.entity(layer_entity).add_child(blob);
            }
        }
    }
    
    add_decorative_elements(commands, layer_entity, rng, level, z_layer, width, height);
}

fn add_decorative_elements(
    commands: &mut Commands,
    parent: Entity,
    rng: &mut StdRng,
    level: u32,
    z_layer: f32,
    width: f32,
    height: f32,
) {
    match level {
        2 => {
            if z_layer == -20.0 { 
                let sun = commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.9, 0.4, 0.2), 
                        custom_size: Some(Vec2::splat(25.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(width * 0.3, height * 0.7, z_layer - 0.1)),
                    ..default()
                }).id();
                
                commands.entity(sun).insert(BackgroundElement {
                    level,
                    depth: z_layer - 0.1,
                });
                
                commands.entity(parent).add_child(sun);
                
                for i in 0..12 {
                    let angle = i as f32 * std::f32::consts::PI / 6.0;
                    let distance = rng.gen_range(15.0..20.0);
                    let ray_x = width * 0.3 + angle.cos() * distance;
                    let ray_y = height * 0.7 + angle.sin() * distance;
                    let ray_length = rng.gen_range(5.0..10.0);
                    
                    let ray = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.9, 0.3, 0.1, 0.6),
                            custom_size: Some(Vec2::new(1.0, ray_length)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(ray_x, ray_y, z_layer - 0.05))
                            .with_rotation(Quat::from_rotation_z(angle)),
                        ..default()
                    }).id();
                    
                    commands.entity(ray).insert(BackgroundElement {
                        level,
                        depth: z_layer - 0.05,
                    });
                    
                    commands.entity(parent).add_child(ray);
                }
            }
            if z_layer == -15.0 {
                for _ in 0..4 {
                    let x = rng.gen_range(0.0..width);
                    let height = rng.gen_range(15.0..30.0);
                    let width = height * 1.5;

                    let volcano = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.3, 0.1, 0.1, 0.8),
                            custom_size: Some(Vec2::new(width, height)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, height/2.0, z_layer - 0.1)),
                        ..default()
                    }).id();
                    
                    commands.entity(volcano).insert(BackgroundElement {
                        level,
                        depth: z_layer - 0.1,
                    });
                    
                    commands.entity(parent).add_child(volcano);
                    
                    let crater = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.9, 0.4, 0.1, 0.7),
                            custom_size: Some(Vec2::new(width * 0.3, height * 0.1)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, height * 0.9, z_layer - 0.05)),
                        ..default()
                    }).id();
                    
                    commands.entity(crater).insert(BackgroundElement {
                        level,
                        depth: z_layer - 0.05,
                    });
                    
                    commands.entity(parent).add_child(crater);
                }
            }

            if z_layer == -10.0 {
                for _ in 0..5 {
                    let x = rng.gen_range(0.0..width);
                    let y = rng.gen_range(0.0..height/3.0);
                    let stream_width = rng.gen_range(1.0..3.0);
                    let stream_height = rng.gen_range(5.0..15.0);
                    
                    let lava = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 0.5, 0.0, 0.8),
                            custom_size: Some(Vec2::new(stream_width, stream_height)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y, z_layer - 0.05)),
                        ..default()
                    }).id();
                    
                    commands.entity(lava).insert(BackgroundElement {
                        level,
                        depth: z_layer - 0.05,
                    });
                    
                    commands.entity(parent).add_child(lava);
                }

                for _ in 0..10 {
                    let x = rng.gen_range(0.0..width);
                    let y = rng.gen_range(0.0..height/2.0);
                    let rock_size = rng.gen_range(2.0..5.0);
                    
                    let rock = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.3, 0.2, 0.2, 0.9),
                            custom_size: Some(Vec2::new(rock_size, rock_size * 1.5)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y, z_layer - 0.08))
                            .with_rotation(Quat::from_rotation_z(rng.gen_range(-0.3..0.3))),
                        ..default()
                    }).id();
                    
                    commands.entity(rock).insert(BackgroundElement {
                        level,
                        depth: z_layer - 0.08,
                    });
                    
                    commands.entity(parent).add_child(rock);
                }
            }
        },
        3 => {
            if z_layer == -20.0 { 
                for _ in 0..8 {
                    let x = rng.gen_range(0.0..width);
                    let mountain_height = rng.gen_range(20.0..40.0);
                    let mountain_width = mountain_height * 2.0;
                    
                    let mountain = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.2, 0.3, 0.2, 0.7),
                            custom_size: Some(Vec2::new(mountain_width, mountain_height)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, mountain_height/2.0, z_layer - 0.1)),
                        ..default()
                    }).id();
                    
                    commands.entity(mountain).insert(BackgroundElement {
                        level,
                        depth: z_layer - 0.1,
                    });
                    
                    commands.entity(parent).add_child(mountain);
                }
                
                let sun = commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.9, 0.9, 0.7, 0.7),
                        custom_size: Some(Vec2::splat(15.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(width * 0.7, height * 0.8, z_layer - 0.05)),
                    ..default()
                }).id();
                
                commands.entity(sun).insert(BackgroundElement {
                    level,
                    depth: z_layer - 0.05,
                });
                
                commands.entity(parent).add_child(sun);
            }
            
            if z_layer == -15.0 { 
                for _ in 0..20 {
                    let x = rng.gen_range(0.0..width);
                    let y = rng.gen_range(height * 0.3..height * 0.7);
                    let tree_size = rng.gen_range(5.0..15.0);
                    
                    let tree_top = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.1, 0.4, 0.1, 0.8),
                            custom_size: Some(Vec2::splat(tree_size)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y, z_layer - 0.1)),
                        ..default()
                    }).id();
                    
                    commands.entity(tree_top).insert(BackgroundElement {
                        level,
                        depth: z_layer - 0.1,
                    });
                    
                    commands.entity(parent).add_child(tree_top);
                }
            }
            
            if z_layer == -10.0 { 
                for _ in 0..15 {
                    let x = rng.gen_range(0.0..width);
                    let y = rng.gen_range(0.0..height/2.0);
                    let trunk_height = rng.gen_range(5.0..12.0);
                    let trunk_width = trunk_height * 0.2;
                    let trunk = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.3, 0.2, 0.1),
                            custom_size: Some(Vec2::new(trunk_width, trunk_height)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y, z_layer - 0.1)),
                        ..default()
                    }).id();
                    
                    commands.entity(trunk).insert(BackgroundElement {
                        level,
                        depth: z_layer - 0.1,
                    });
                    let top_size = trunk_height * rng.gen_range(0.8..1.2);
                    let top = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.1, 0.4, 0.1), 
                            custom_size: Some(Vec2::new(top_size, top_size)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y + trunk_height * 0.8, z_layer - 0.05)),
                        ..default()
                    }).id();
                    
                    commands.entity(top).insert(BackgroundElement {
                        level,
                        depth: z_layer - 0.05,
                    });
                    
                    commands.entity(parent).add_child(trunk);
                    commands.entity(parent).add_child(top);
                    
                    for i in 0..3 {
                        let bush_x = x + rng.gen_range(-trunk_width*2.0..trunk_width*2.0);
                        let bush_size = rng.gen_range(1.0..2.0);
                        
                        let bush = commands.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgb(0.15, 0.35, 0.1),
                                custom_size: Some(Vec2::splat(bush_size)),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(bush_x, y, z_layer)),
                            ..default()
                        }).id();
                        
                        commands.entity(bush).insert(BackgroundElement {
                            level,
                            depth: z_layer,
                        });
                        
                        commands.entity(parent).add_child(bush);
                    }
                }
            }
        },
        _ => {
            let element_count = 15;
            for _ in 0..element_count {
                let x = rng.gen_range(-width/2.0..width/2.0);
                let y = rng.gen_range(height/3.0..height);
                let cloud_width = rng.gen_range(5.0..15.0);
                let cloud_height = rng.gen_range(2.0..4.0);
                
                let cloud = commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.4, 0.4, 0.8, 0.4),
                        custom_size: Some(Vec2::new(cloud_width, cloud_height)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, z_layer - 0.1))
                        .with_rotation(Quat::from_rotation_z(rng.gen_range(-0.1..0.1))),
                    ..default()
                }).id();
                
                commands.entity(cloud).insert(BackgroundElement {
                    level,
                    depth: z_layer - 0.1,
                });
                
                commands.entity(parent).add_child(cloud);
            }
        }
    }
}

const TILE_SIZE: f32 = 1.0;

pub fn update_parallax(
    player_query: Query<&Transform, With<crate::player::Player>>,
    mut parallax_query: Query<(&ParallaxLayer, &mut Transform), (With<BackgroundElement>, Without<crate::player::Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (layer, mut transform) in parallax_query.iter_mut() {
            transform.translation.x = player_transform.translation.x * layer.speed_factor;
        }
    }
}

pub struct BackgroundGeneratorPlugin;

impl Plugin for BackgroundGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
               OnEnter(GameState::Level1), generate_level_background.clone()
           )
           .add_systems(
               OnEnter(GameState::Level2), generate_level_background.clone()
           )
           .add_systems(
               OnEnter(GameState::Level3), generate_level_background.clone()
           )
           .add_systems(Update, update_parallax.after(crate::player::player_movement_system));
    }
}