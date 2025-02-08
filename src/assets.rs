use bevy::prelude::*;

#[derive(Resource, Default, Clone)]
pub struct LevelAssets {
    pub current_level: u32,
    pub background_textures: Vec<Handle<Image>>,
    pub block_textures: Vec<Handle<Image>>,
}

pub fn load_level_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: u32,
) -> LevelAssets {
    let mut assets = LevelAssets {
        current_level,
        background_textures: Vec::new(),
        block_textures: Vec::new(),
    };
    
    match current_level {
        2 => {
            let block_texture = asset_server.load("blocks/2.png");
            assets.block_textures.push(block_texture);
        },
        2 => {
            let block_texture = asset_server.load("blocks/3.png");
            assets.block_textures.push(block_texture);
        },
        2 => {
            let block_texture = asset_server.load("blocks/4.png");
            assets.block_textures.push(block_texture);
        },
        _ => {
            let block_texture = asset_server.load("blocks/1.png");
            assets.block_textures.push(block_texture);
        }
    }
    
    commands.insert_resource(assets.clone());
    assets
}
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelAssets>()
           .add_systems(Startup, |asset_server: Res<AssetServer>, mut commands: Commands| {
               load_level_assets(commands, asset_server, 1);
           });
    }
}