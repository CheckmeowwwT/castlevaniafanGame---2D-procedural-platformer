# CastlevaniaFanGame — Procedural Platformer in Rust/Bevy

This is a 2D side-scrolling platformer I built using Bevy. It is platformer with procedural level generation, with seeded RNG, dash mechanics, sprite sheet animation, parallax backgrounds via Perlin noise, level transitions with fade overlays, game over/restart, and BFS-based level solvability validation.

**Note:** I used Bevy 0.12/0.13. Bevy has changed significantly since then and so this probably will not compile on current versions without migration work. The project is abandoned. Please peruse through the remains.

## What it does

The game generates a procedural platformer level on startup using a seeded RNG. The level consists of a 200x32 tile grid populated with 10 different platform patterns (normal, staggered, floating, zigzag, challenge, ascending, descending, spiral, maze, waterfall). Each pattern places blocks, fire blocks, and exit blocks according to its own rules. A start platform and an exit platform are always guaranteed.

The player has standard platformer movement plus a dash mechanic with cooldown. Sprite sheet animation handles idle, walk, jump, and dash states using a 4-frame atlas. Horizontal and vertical tile collision is checked separately. Basically, horizontal checks a column of tiles at the player's edge, and vertical checks the tile under the player's feet. There is a jump buffer (0.25s input window) so jumps feel responsive.

When the player reaches the end of a level, a fade-to-black transition plays, the map regenerates with a new seed derived from the base seed + level number, assets reload for the new level's block textures, and the player respawns. There are 3 levels. Falling below y=0 for more than 0.5 seconds triggers a game over screen with restart on R.

Backgrounds are generated per-level using Perlin noise across 3 parallax layers at different z-depths. Each level has its own color palette and decorative elements. Level 1 gets clouds, level 2 gets a sun with rays and rock formations, level 3 gets mountains and trees with bushes. There is also an animated tree sprite that activates via parallax when the player passes a threshold. I wanted to build more levels then.

## Architecture

```
src/
├── main.rs                  — app setup, window, camera
├── lib.rs                   — module declarations
├── control.rs               — keyboard input → PlayerInput resource (movement, jump, dash)
├── player.rs                — player entity, movement, dash, sprite animation, camera follow
├── map.rs                   — tile grid, seeded procedural generation, 10 platform patterns, rendering
├── assets.rs                — per-level texture loading
├── background.rs            — Perlin noise background with texture fallback
├── background_generator.rs  — per-level parallax layers with decorative elements
├── trees.rs                 — animated parallax tree sprites
├── level.rs                 — multi-level state machine, fade transitions, level change events
└── game_over.rs             — death detection, game over UI, restart

tests/
├── level_validation.rs      — verifies start and end platforms exist after generation
├── path_validation.rs       — BFS pathfinding to verify level is completable given player capabilities
└── player_movement.rs       — tests dash initiation, cooldown, and re-dash timing
```

The ECS is split into plugins. ControlPlugin reads keyboard state into a PlayerInput resource. PlayerPlugin consumes it for movement and owns the animation system. MapPlugin handles generation and rendering. LevelPlugin manages state transitions between levels. Each system runs independently — input, physics, rendering, and game state are decoupled.

## Level generation

Levels are generated from a seed (SystemTime epoch seconds). Each level derives its own seed as base_seed + level_number \* 100, so the same base seed always produces the same sequence of levels.

The generator places a guaranteed start platform (15 tiles wide), then iterates through the level length selecting random patterns from the pattern pool. Patterns include things like zigzag single-tile jumps, floating island clusters, challenge sections with gaps, ascending/descending staircases, spiral structures, and waterfall columns of fire blocks. If a platform gets too high above base height, the generator inserts support pillars. The level ends with a guaranteed exit platform marked with ExitBlock tiles.

Fire blocks are scattered probabilistically within patterns (15-40% chance depending on pattern type). They were meant to flicker but I ran out of time.

## Tests

Three test files validate the generated levels:

**level_validation** — generates a level and checks that block tiles exist at the start platform and exit block tiles exist at the end platform across the expected x-ranges.

**path_validation** — runs BFS from player spawn to the end region, modeling the player's jump height, jump width, and dash distance. It checks that a valid path exists across 3 different seeds. The BFS considers walking (must have ground below), jumping (various arcs with path clearance checks), and dashing (horizontal movement with obstacle checks).

**player_movement** — spawns a player entity in a minimal Bevy app with a flat floor, triggers a dash input, and verifies: dash velocity exceeds normal speed, dash timer is active, dash ends after duration, cooldown prevents re-dashing, and re-dash works after cooldown expires.

Note: These are simplistic. I did not have the time to write them properly.

## Dependencies

- `bevy` — ECS game engine
- `noise` — Perlin noise for backgrounds
- `rand` — seeded RNG for level generation
- `pathfinding` — used in the first version, retained as dependency

## Status

Playable prototype with 3 levels. Core loop worked: run, jump, dash through procedural terrain, transition between levels, die and restart. Things I wanted but did not get to: fire block flickering animation, exit block fade effect, actual enemy entities (the first version had patrol AI but I dropped enemies when I rebuilt the level system), and sub-weapon pickups. I have decided to redo this in a better engine.