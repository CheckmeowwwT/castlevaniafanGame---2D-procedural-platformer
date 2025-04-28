// tests/player_movement.rs
use bevy::prelude::*;
use CastleVaniaGame::{
    player::{Player, PLAYER_SPEED, JUMP_FORCE, GRAVITY, player_movement_system},
    control::PlayerInput,
    map::{Map, TileType},
};

#[test]
fn test_player_dash_mechanics() {
    // Create a minimal Bevy app for testing
    let mut app = App::new();
    
    // Add time resource
    app.init_resource::<Time>();
    
    // Add needed resources
    app.insert_resource(PlayerInput {
        movement: Vec2::new(1.0, 0.0), // Moving right
        jump: false,
        dash: true, // Trying to dash
    });
    
    // Create a simple flat map for testing
    let mut tiles = vec![TileType::Empty; 200 * 32];
    
    // Add a floor at y=10
    for x in 0..200 {
        tiles[10 * 200 + x] = TileType::Block(0);
    }
    
    app.insert_resource(Map {
        surfaces: vec![10; 200],
        tiles,
        current_level: 1,
        level_length: 180,
    });
    
    // Spawn a player entity
    let player_entity = app.world.spawn((
        Player {
            velocity: Vec2::ZERO,
            is_grounded: true,
            jump_request: 0.0,
            is_dashing: false,
            dash_timer: 0.0,
            dash_cooldown: 0.0,
            facing: 1.0, // Facing right
            anim_state: CastleVaniaGame::player::AnimationState::Idle,
            anim_timer: 0.0,
        },
        Transform::from_xyz(10.0, 11.0, 0.0), // Just above the floor
    )).id();
    
    // Add the player movement system
    app.add_systems(Update, player_movement_system);
    
    // Run one update to initiate the dash
    app.update();
    
    // Check if the player is dashing
    let player = app.world.entity(player_entity).get::<Player>().unwrap();
    assert!(player.is_dashing, "Player should be dashing after input");
    
    // The dash velocity should be higher than normal movement
    assert!(player.velocity.x > PLAYER_SPEED, 
            "Dash velocity should be higher than normal speed. Current: {}", 
            player.velocity.x);
    
    // Now test dash cooldown
    let dash_timer_after_dash = player.dash_timer;
    assert!(dash_timer_after_dash > 0.0, "Dash timer should be active");
    
    // Turn off dash input
    app.world.resource_mut::<PlayerInput>().dash = false;
    
    // Run several updates to complete the dash
    for _ in 0..10 {
        app.update();
    }
    
    // After several updates, player should no longer be dashing
    let player = app.world.entity(player_entity).get::<Player>().unwrap();
    assert!(!player.is_dashing, "Player should no longer be dashing");
    
    // Cooldown should be active
    assert!(player.dash_cooldown > 0.0, "Dash cooldown should be active");
    
    // Try to dash again (should fail due to cooldown)
    app.world.resource_mut::<PlayerInput>().dash = true;
    app.update();
    
    let player = app.world.entity(player_entity).get::<Player>().unwrap();
    assert!(!player.is_dashing, "Player should not be able to dash during cooldown");
    
    // Wait for cooldown to expire
    for _ in 0..20 {
        app.update();
    }
    
    // Now should be able to dash again
    app.world.resource_mut::<PlayerInput>().dash = true;
    app.update();
    
    let player = app.world.entity(player_entity).get::<Player>().unwrap();
    assert!(player.is_dashing, "Player should be able to dash after cooldown expired");
}