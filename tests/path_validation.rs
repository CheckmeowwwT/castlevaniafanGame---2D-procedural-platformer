// tests/path_validation.rs
use bevy::prelude::*;
use CastleVaniaGame::{
    map::{Map, TileType, generate_platformer_level, LevelSeed, PLAYER_START_X, PLAYER_START_Y},
    assets::LevelAssets,
};
use std::collections::{HashSet, VecDeque};

#[test]
fn test_level_can_be_completed() {
    // Create a minimal Bevy app for testing
    let mut app = App::new();
    
    // Initialize resources with different seeds to test multiple layouts
    let seeds = [12345, 67890, 24680];
    
    for &seed in &seeds {
        // Set up a new app for each seed
        let mut app = App::new();
        
        let level_seed = LevelSeed { seed };
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
        
        // Generate the level
        app.add_systems(Update, |
            mut map: ResMut<Map>,
            level_assets: Res<LevelAssets>,
            level_seed: Res<LevelSeed>,
        | {
            generate_platformer_level(&mut map, &level_assets, &level_seed);
        });
        
        app.update();
        
        // Get the generated map
        let map = app.world.resource::<Map>();
        
        // Define player capabilities
        let max_jump_height = 6;  // Maximum height player can jump
        let max_jump_width = 8;   // Maximum horizontal distance in one jump
        let can_dash = true;      // Player can dash
        let dash_distance = 5;    // Horizontal dash distance
        
        // Run pathfinding to check if level is completable
        let is_completable = check_level_completable(
            &map, 
            max_jump_height, 
            max_jump_width, 
            can_dash, 
            dash_distance
        );
        
        println!("Level with seed {} completable: {}", seed, is_completable);
        
        // The level should be completable
        assert!(is_completable, "Level with seed {} should be completable", seed);
    }
}

// A simplified pathfinding algorithm to check if the level can be completed
fn check_level_completable(
    map: &Map,
    max_jump_height: i32,
    max_jump_width: i32,
    can_dash: bool,
    dash_distance: i32,
) -> bool {
    // Start position (player spawn)
    let start_x = PLAYER_START_X;
    let start_y = PLAYER_START_Y;
    
    // End position (end of level)
    let end_region_start = (map.level_length - 15) as i32;
    
    // Use a breadth-first search to find a path
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    // Add start position
    queue.push_back((start_x, start_y));
    visited.insert((start_x, start_y));
    
    // BFS loop
    while let Some((x, y)) = queue.pop_front() {
        // Check if we've reached the end region
        if x >= end_region_start {
            return true;
        }
        
        // Try walking left/right
        for dx in [-1, 1] {
            let new_x = x + dx;
            let new_y = y;
            
            // Only consider if not visited and not a solid block
            if !visited.contains(&(new_x, new_y)) && 
               !map.get_tile(new_x, new_y).is_collidable() &&
               map.get_tile(new_x, new_y - 1).is_collidable() { // Must be standing on solid ground
                queue.push_back((new_x, new_y));
                visited.insert((new_x, new_y));
            }
        }
        
        // Try jumping (simplification - try various jump arcs)
        for jump_height in 1..=max_jump_height {
            for jump_dx in -max_jump_width..=max_jump_width {
                let new_x = x + jump_dx;
                let new_y = y + jump_height;
                
                // Don't try impossible jumps
                if jump_dx.abs() > max_jump_width * jump_height / max_jump_height {
                    continue;
                }
                
                // Check if landing spot is valid (not solid and has solid ground beneath)
                if map.in_bounds(new_x, new_y) &&
                   !map.get_tile(new_x, new_y).is_collidable() &&
                   map.get_tile(new_x, new_y - 1).is_collidable() &&
                   !visited.contains(&(new_x, new_y)) {
                    
                    // Simple path check - just make sure there are no solid blocks in a straight line
                    // This is a simplification - real jumps would follow an arc
                    let mut path_clear = true;
                    for i in 1..jump_dx.abs() {
                        let check_x = x + i * jump_dx.signum();
                        let check_y = y + i * jump_height / jump_dx.abs();
                        if map.get_tile(check_x, check_y).is_collidable() {
                            path_clear = false;
                            break;
                        }
                    }
                    
                    if path_clear {
                        queue.push_back((new_x, new_y));
                        visited.insert((new_x, new_y));
                    }
                }
            }
        }
        
        // Try dash jumps if enabled
        if can_dash {
            for dir in [-1, 1] {
                let dash_x = x + dir * dash_distance;
                
                // Check if dash landing is valid
                if map.in_bounds(dash_x, y) && 
                   !map.get_tile(dash_x, y).is_collidable() &&
                   !visited.contains(&(dash_x, y)) {
                    
                    // Check if dash path is clear
                    let mut path_clear = true;
                    for i in 1..dash_distance {
                        let check_x = x + i * dir;
                        if map.get_tile(check_x, y).is_collidable() {
                            path_clear = false;
                            break;
                        }
                    }
                    
                    if path_clear {
                        queue.push_back((dash_x, y));
                        visited.insert((dash_x, y));
                    }
                }
            }
        }
    }

    false
}