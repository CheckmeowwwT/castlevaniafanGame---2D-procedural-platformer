use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::background::spawn_background;
use crate::assets::LevelAssets;

pub const TILE_SIZE: f32 = 1.0;
pub const PLATFORM_MIN_Y: usize = 3;
pub const PLATFORM_MAX_Y: usize = 25;
pub const MAP_WIDTH: usize = 200;
pub const MAP_HEIGHT: usize = 32;
pub const PLAYER_START_X: i32 = 3;
pub const PLAYER_START_Y: i32 = PLATFORM_MAX_Y as i32 + 3;
pub const BLOCK_SCALE: f32 = 1.0/64.0;

pub const LEVEL1_LENGTH: usize = 180;
pub const LEVEL2_LENGTH: usize = 190;
pub const LEVEL3_LENGTH: usize = 195;

#[derive(Resource, Default)]
pub struct LevelSeed {
    pub seed: u64,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Empty,
    Block(usize), 
    ExitBlock(usize), 
    FireBlock(usize), 
}

impl TileType {
    pub fn is_collidable(self) -> bool {
        match self {
            TileType::Empty => false,
            TileType::Block(_) => true,
            TileType::ExitBlock(_) => true, // was meant to create a fade effect
            TileType::FireBlock(_) => true, // was meant to flicker but i ran out of time
        }
    }
    
    }

#[derive(Resource, Clone)]
pub struct Map {
    pub surfaces: Vec<usize>, 
    pub tiles: Vec<TileType>,
    pub current_level: u32,
    pub level_length: usize,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y.clamp(0, MAP_HEIGHT as i32 - 1) as usize) * MAP_WIDTH + x.clamp(0, MAP_WIDTH as i32 - 1) as usize
    }
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < MAP_WIDTH as i32 && y >= 0 && y < MAP_HEIGHT as i32
    }
    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        if self.in_bounds(x, y) {
            self.tiles[self.xy_idx(x, y)]
        } else {
            TileType::Empty
        }
    }
    
}

pub fn generate_platformer_level(map: &mut Map, level_assets: &LevelAssets,  level_seed: &Res<LevelSeed>) {
    let seed = level_seed.seed + (map.current_level as u64 * 100);
    let mut rng = StdRng::seed_from_u64(seed);
    
    for t in map.tiles.iter_mut() {
        *t = TileType::Empty;
    }
    
    let texture_idx = 0;

    let level_length = match map.current_level {
        2 => LEVEL2_LENGTH,
        3 => LEVEL3_LENGTH,
        _ => LEVEL1_LENGTH,
    };
    
    map.level_length = level_length;
    
    let base_platform_y = PLAYER_START_Y as usize - 1;

    let start_platform_length = 15; // always the same start point
    for x in 0..start_platform_length {
        let idx = map.xy_idx(x as i32, base_platform_y as i32);
        map.tiles[idx] = TileType::Block(texture_idx);
        
        let below_idx = map.xy_idx(x as i32, (base_platform_y - 1) as i32);
        map.tiles[below_idx] = TileType::Block(texture_idx);
    }
    let mut current_x = start_platform_length;
    let end_x = level_length - 15; 
    let mut last_platform_y = base_platform_y;

    let patterns = ["normal", "staggered", "floating", "zigzag", "challenge", 
                   "ascending", "descending", "spiral", "maze", "waterfall"];
    
    while current_x < end_x {
        let pattern = patterns[rng.gen_range(0..patterns.len())];
        
        match pattern {
            "normal" => {
                let platform_length = rng.gen_range(3..10);
                let gap = rng.gen_range(2..5);
                current_x += gap;
                let y_max_diff = if last_platform_y > base_platform_y + 5 { -5 } else { 8 };
                let y_min_diff = if last_platform_y < base_platform_y - 3 { 3 } else { -5 };
                let y_offset = rng.gen_range(y_min_diff..y_max_diff);
                
                let platform_y = (last_platform_y as i32 + y_offset)
                    .clamp(PLATFORM_MIN_Y as i32, PLATFORM_MAX_Y as i32) as usize;
                for x in current_x..(current_x + platform_length).min(end_x) {
                    let idx = map.xy_idx(x as i32, platform_y as i32);
                    if rng.gen_bool(0.15) {
                        map.tiles[idx] = TileType::FireBlock(texture_idx);
                    } else {
                        map.tiles[idx] = TileType::Block(texture_idx);
                    }
                }
                
                current_x += platform_length;
                last_platform_y = platform_y;
            },
            "staggered" => {
                let step_count = rng.gen_range(3..6);
                let direction = if rng.gen_bool(0.5) { 1 } else { -1 };
                
                let mut step_y = last_platform_y;
                
                for i in 0..step_count {
                    let step_x = current_x + i * 3;
                    step_y = ((step_y as i32) + direction)
                        .clamp(PLATFORM_MIN_Y as i32, PLATFORM_MAX_Y as i32) as usize;
                    let idx = map.xy_idx(step_x as i32, step_y as i32);
                    
                    if rng.gen_bool(0.2) {
                        map.tiles[idx] = TileType::FireBlock(texture_idx);
                    } else {
                        map.tiles[idx] = TileType::Block(texture_idx);
                    }
                }
                
                current_x += step_count * 3;
                last_platform_y = step_y;
            },
            "floating" => {
                let platform_count = rng.gen_range(2..5);
                
                for i in 0..platform_count {
                    let platform_x = current_x + i * 4;
                    let y_offset = rng.gen_range(-4..5);
                    let platform_y = (last_platform_y as i32 + y_offset)
                        .clamp(PLATFORM_MIN_Y as i32, PLATFORM_MAX_Y as i32) as usize;
                    
                    let width = rng.gen_range(2..4);
                    for x in 0..width {
                        if platform_x + x < end_x {
                            let idx = map.xy_idx((platform_x + x) as i32, platform_y as i32);
                            
                            if rng.gen_bool(0.3) {
                                map.tiles[idx] = TileType::FireBlock(texture_idx);
                            } else {
                                map.tiles[idx] = TileType::Block(texture_idx);
                            }
                        }
                    }
                }
                
                current_x += platform_count * 4;
            },
            "zigzag" => {
                let steps = rng.gen_range(5..9);
                let mut zig_y = last_platform_y;
                let mut direction = if rng.gen_bool(0.5) { 1 } else { -1 };
                
                for i in 0..steps {
                    direction *= -1; 
                    zig_y = ((zig_y as i32) + direction * 2)
                        .clamp(PLATFORM_MIN_Y as i32, PLATFORM_MAX_Y as i32) as usize;
                    
                    let zig_x = current_x + i * 2;
                    if zig_x < end_x {
                        let idx = map.xy_idx(zig_x as i32, zig_y as i32);
                        map.tiles[idx] = TileType::Block(texture_idx);
                    }
                }
                
                current_x += steps * 2;
                last_platform_y = zig_y;
            },
            "challenge" => {
                let length = rng.gen_range(10..20);
                let challenge_y = last_platform_y;
                for x in 0..length {
                    if rng.gen_bool(0.6) { 
                        let pos_x = current_x + x;
                        if pos_x < end_x {
                            let idx = map.xy_idx(pos_x as i32, challenge_y as i32);
                            if rng.gen_bool(0.4) {
                                map.tiles[idx] = TileType::FireBlock(texture_idx);
                            } else {
                                map.tiles[idx] = TileType::Block(texture_idx);
                            }
                        }
                    }
                }
                
                current_x += length;
            },
            "ascending" => {
                let steps = rng.gen_range(4..8);
                let step_height = rng.gen_range(1..3);
                let step_width = rng.gen_range(2..5);
                let gap = rng.gen_range(1..3);
                
                let mut asc_y = last_platform_y;
                
                for i in 0..steps {
                    let step_x = current_x + i * (step_width + gap);
                    asc_y = ((asc_y as i32) + step_height)
                        .clamp(PLATFORM_MIN_Y as i32, PLATFORM_MAX_Y as i32) as usize;
                    for x in 0..step_width {
                        if step_x + x < end_x {
                            let idx = map.xy_idx((step_x + x) as i32, asc_y as i32);
                            if rng.gen_bool(0.1) {
                                map.tiles[idx] = TileType::FireBlock(texture_idx);
                            } else {
                                map.tiles[idx] = TileType::Block(texture_idx);
                            }
                        }
                    }
                }
                
                current_x += steps * (step_width + gap);
                last_platform_y = asc_y;
            },
            "descending" => {
                let steps = rng.gen_range(4..8);
                let step_height = rng.gen_range(1..3);
                let step_width = rng.gen_range(2..5);
                let gap = rng.gen_range(1..3);
                
                let mut desc_y = last_platform_y;
                
                for i in 0..steps {
                    let step_x = current_x + i * (step_width + gap);
                    desc_y = ((desc_y as i32) - step_height)
                        .clamp(PLATFORM_MIN_Y as i32, PLATFORM_MAX_Y as i32) as usize;

                    for x in 0..step_width {
                        if step_x + x < end_x {
                            let idx = map.xy_idx((step_x + x) as i32, desc_y as i32);
                            if rng.gen_bool(0.1) {
                                map.tiles[idx] = TileType::FireBlock(texture_idx);
                            } else {
                                map.tiles[idx] = TileType::Block(texture_idx);
                            }
                        }
                    }
                }
                
                current_x += steps * (step_width + gap);
                last_platform_y = desc_y;
            },
            "spiral" => {
                let radius = rng.gen_range(5..10);
                let turns = rng.gen_range(1.0..2.0);
                let steps = rng.gen_range(8..16);
                
                let center_y = last_platform_y;
                let center_x = current_x + radius;
                
                for i in 0..steps {
                    let angle = (i as f32 / steps as f32) * turns * 2.0 * std::f32::consts::PI;
                    let x = center_x + (radius as f32 * angle.cos()) as usize;
                    let y = center_y + (radius as f32 * angle.sin()) as usize;
                    
                    if x < end_x {
                        let idx = map.xy_idx(x as i32, y as i32);
                        map.tiles[idx] = TileType::Block(texture_idx);
                        if rng.gen_bool(0.2) {
                            let fire_x = x + 1;
                            if fire_x < end_x {
                                let fire_idx = map.xy_idx(fire_x as i32, y as i32);
                                map.tiles[fire_idx] = TileType::FireBlock(texture_idx);
                            }
                        }
                    }
                }
                
                current_x += radius * 2;
            },
            "maze" => {
                let width = rng.gen_range(10..20);
                let height = rng.gen_range(3..6);
                
                let base_y = last_platform_y - height/2;
                for x in 0..width {
                    if current_x + x < end_x {
                        let idx = map.xy_idx((current_x + x) as i32, base_y as i32);
                        map.tiles[idx] = TileType::Block(texture_idx);
                    }
                }
                for h in 1..height {
                    for x in 0..width {
                        if x % 3 == 0 && rng.gen_bool(0.7) {
                            if current_x + x < end_x {
                                let idx = map.xy_idx((current_x + x) as i32, (base_y + h) as i32);
                                
                                if rng.gen_bool(0.2) {
                                    map.tiles[idx] = TileType::FireBlock(texture_idx);
                                } else {
                                    map.tiles[idx] = TileType::Block(texture_idx);
                                }
                            }
                        }
                    }
                }
                for h in 1..height {
                    if rng.gen_bool(0.5) {
                        let bridge_x = rng.gen_range(0..width-3);
                        for x in 0..3 {
                            if current_x + bridge_x + x < end_x {
                                let idx = map.xy_idx((current_x + bridge_x + x) as i32, (base_y + h) as i32);
                                map.tiles[idx] = TileType::Block(texture_idx);
                            }
                        }
                    }
                }
                
                current_x += width;
                last_platform_y = base_y + height;
            },

"waterfall" => {
    let width = rng.gen_range(1..3);
    let height = rng.gen_range(5..10);
    for x_offset in -2i32..(width as i32 + 3) {
        let x = current_x as i32 + x_offset;
        if x >= 0 && (x as usize) < end_x {
            let idx = map.xy_idx(x, last_platform_y as i32);
            map.tiles[idx] = TileType::Block(texture_idx);
        }
    }
    for h in 1..height {
        for x in 0..width {
            if current_x + x < end_x {
                let idx = map.xy_idx((current_x + x) as i32, (last_platform_y - h) as i32);
                map.tiles[idx] = TileType::FireBlock(texture_idx);
            }
        }
    }
    
    current_x += width + 5;
},
            _ => {}
        }
        if current_x < end_x && last_platform_y > base_platform_y + 4 {
            let pillar_x = current_x - 4;
            if pillar_x > start_platform_length {
                for y in base_platform_y..=last_platform_y {
                    let idx = map.xy_idx(pillar_x as i32, y as i32);
                    map.tiles[idx] = TileType::Block(texture_idx);
                }
            }
        }
    }
    
    let end_platform_length = 15;
    let end_platform_start = level_length - end_platform_length;
    
    for x in end_platform_start..level_length {
        let idx = map.xy_idx(x as i32, base_platform_y as i32);
        map.tiles[idx] = TileType::ExitBlock(texture_idx);

        let below_idx = map.xy_idx(x as i32, (base_platform_y - 1) as i32);
        map.tiles[below_idx] = TileType::ExitBlock(texture_idx);
    }
    for y in (base_platform_y + 1)..(base_platform_y + 5) {
        let x = end_platform_start;
        let idx = map.xy_idx(x as i32, y as i32);
        map.tiles[idx] = TileType::ExitBlock(texture_idx);
        
        let end_x = level_length - 1;
        let end_idx = map.xy_idx(end_x as i32, y as i32);
        map.tiles[end_idx] = TileType::ExitBlock(texture_idx);
    }

    for x in 0..MAP_WIDTH {
        map.surfaces[x] = (0..MAP_HEIGHT)
            .rev()
            .find(|&y| map.tiles[map.xy_idx(x as i32, y as i32)].is_collidable())
            .unwrap_or(0);
    }
}

#[derive(Component)]
pub struct MapRenderer;

#[derive(Component)]
pub struct CollisionBlock;

#[derive(Component)]
pub struct ExitBlock;

#[derive(Component)]
pub struct FireBlock; 

pub fn render_map(
    mut commands: Commands,
    map: Res<Map>,
    level_assets: Res<LevelAssets>,
    query: Query<Entity, With<MapRenderer>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    if level_assets.block_textures.is_empty() {
        info!("No block textures found! Cannot render map.");
        return;
    }
    
    let map_entity = commands.spawn((SpatialBundle::default(), MapRenderer)).id();
    let mut block_count = 0;
    let mut exit_block_count = 0;
    let mut fire_block_count = 0;
    
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let idx = y * MAP_WIDTH + x;
            match map.tiles[idx] {
                TileType::Empty => {}
                TileType::Block(texture_idx) => {
                    let pos = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
                    let safe_tex_idx = texture_idx % level_assets.block_textures.len();
                    let scale_factor = match map.current_level {
                        1 => BLOCK_SCALE, 
                        _ => 1.0/84.0, 
                    };
                    
                    let block_entity = commands.spawn((
                        SpriteBundle {
                            texture: level_assets.block_textures[safe_tex_idx].clone(),
                            transform: Transform::from_translation(pos)
                                .with_scale(Vec3::splat(scale_factor)),
                            ..default()
                        },
                        CollisionBlock,
                    )).id();
                    
                    commands.entity(block_entity).set_parent(map_entity);
                    block_count += 1;
                },
                TileType::ExitBlock(texture_idx) => {
                    let pos = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
                    let safe_tex_idx = texture_idx % level_assets.block_textures.len();
                    let scale_factor = match map.current_level {
                        1 => BLOCK_SCALE,
                        _ => 1.0/84.0,
                    };
                    
                    let exit_entity = commands.spawn((
                        SpriteBundle {
                            texture: level_assets.block_textures[safe_tex_idx].clone(),
                            transform: Transform::from_translation(pos)
                                .with_scale(Vec3::splat(scale_factor)),
                            sprite: Sprite {
                                color: Color::rgb(1.2, 1.0, 0.4),
                                ..default()
                            },
                            ..default()
                        },
                        CollisionBlock,
                        ExitBlock,
                    )).id();
                    
                    commands.entity(exit_entity).set_parent(map_entity);
                    exit_block_count += 1;
                },
                TileType::FireBlock(texture_idx) => {
                    let pos = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
                    let safe_tex_idx = texture_idx % level_assets.block_textures.len();
                    let scale_factor = match map.current_level {
                        1 => BLOCK_SCALE,
                        _ => 1.0/84.0,
                    };
                    
                    let fire_entity = commands.spawn((
                        SpriteBundle {
                            texture: level_assets.block_textures[safe_tex_idx].clone(),
                            transform: Transform::from_translation(pos)
                                .with_scale(Vec3::splat(scale_factor)),
                            sprite: Sprite {
                                color: Color::rgb(1.5, 0.3, 0.2),
                                ..default()
                            },
                            ..default()
                        },
                        CollisionBlock,
                        FireBlock,
                    )).id();
                    
                    commands.entity(fire_entity).set_parent(map_entity);
                    fire_block_count += 1;
                }
            }
        }
    }
    
}

pub fn generate_map(mut map: ResMut<Map>, level_assets: Res<LevelAssets>, level_seed: Res<LevelSeed>) {
    generate_platformer_level(&mut *map, &level_assets, &level_seed);
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map {
            surfaces: vec![PLATFORM_MIN_Y; MAP_WIDTH],
            tiles: vec![TileType::Empty; MAP_WIDTH * MAP_HEIGHT],
            current_level: 1,
            level_length: LEVEL1_LENGTH,
        })
        .insert_resource(LevelSeed {
            seed: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(std::time::Duration::from_secs(0))
                .as_secs(),
        })
        .add_systems(Startup, (
            spawn_background,
            generate_map,
        ))
        .add_systems(Update, render_map);
    }
}