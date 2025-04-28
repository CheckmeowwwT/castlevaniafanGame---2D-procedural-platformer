
use bevy::prelude::*;
use CastleVaniaGame::{
    map::{Map, TileType, generate_platformer_level, LevelSeed, PLAYER_START_Y},
    assets::LevelAssets,
};

#[test]
fn test_level_has_start_and_end_platforms() {
    let mut app = App::new();
    
    let level_seed = LevelSeed { seed: 12345 };
    app.insert_resource(level_seed);
    
    let level_assets = LevelAssets {
        current_level: 1,
        background_textures: Vec::new(),
        block_textures: Vec::new(),
    };
    app.insert_resource(level_assets);
    
    app.insert_resource(Map {
        surfaces: vec![3; 200],
        tiles: vec![TileType::Empty; 200 * 32],
        current_level: 1,
        level_length: 180,
    });

    app.add_systems(Update, |
        mut map: ResMut<Map>,
        level_assets: Res<LevelAssets>,
        level_seed: Res<LevelSeed>,
    | {
        generate_platformer_level(&mut map, &level_assets, &level_seed);
    });
    app.update();
    let map = app.world.resource::<Map>();
    let base_platform_y = PLAYER_START_Y as usize - 1;
    
    for x in 0..15 {
        assert!(matches!(map.get_tile(x, base_platform_y as i32), TileType::Block(_)), 
                "Start platform missing at x={}, y={}", x, base_platform_y);
    }

    let end_x_start = map.level_length - 15;
    for x in end_x_start..map.level_length {
        assert!(matches!(map.get_tile(x as i32, base_platform_y as i32), TileType::ExitBlock(_)), 
                "End platform missing at x={}, y={}", x, base_platform_y);
    }
}