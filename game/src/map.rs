use bevy::prelude::*;
use std::collections::HashMap;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, setup_level, spawn_tiles).chain());
    }
}

const TILE_SIZE: f32 = 16.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TileKind {
    Grass,
    Dirt,
    Road,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpriteSheetKind {
    TerrainAndFences,
    CityTerrain,
}

#[derive(Resource, Debug)]
struct SpriteSheetRegistry {
    spritesheets: HashMap<SpriteSheetKind, (Handle<Image>, Handle<TextureAtlasLayout>)>,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub kind: TileKind,
    pub spritesheet: SpriteSheetKind,
    pub index: usize, // index insdie the spritesheet
    pub width: u32,   // # of tiles
    pub height: u32,  // # of tiles
}

impl Tile {
    fn grass() -> Self {
        Tile {
            kind: TileKind::Grass,
            spritesheet: SpriteSheetKind::TerrainAndFences,
            index: 0,
            width: 1,
            height: 1,
        }
    }

    fn dirt() -> Self {
        Tile {
            kind: TileKind::Dirt,
            spritesheet: SpriteSheetKind::TerrainAndFences,
            index: 1,
            width: 1,
            height: 1,
        }
    }

    fn road() -> Self {
        Tile {
            kind: TileKind::Road,
            spritesheet: SpriteSheetKind::CityTerrain,
            index: 0,
            width: 1,
            height: 1,
        }
    }

    fn width_px(&self) -> f32 {
        self.width as f32 * TILE_SIZE
    }

    fn height_px(&self) -> f32 {
        self.height as f32 * TILE_SIZE
    }
}

#[derive(Resource)]
pub struct TileGrid {
    pub tiles: HashMap<(i32, i32), Tile>,
    pub width: i32,
    pub height: i32,
}

impl TileGrid {
    fn new(width: i32, height: i32) -> Self {
        TileGrid {
            tiles: HashMap::new(),
            width,
            height,
        }
    }

    fn set_tile(&mut self, col: i32, row: i32, tile: Tile) {
        if col >= 0 && col < self.width && row >= 0 && row < self.height {
            self.tiles.insert((col, row), tile);
        }
    }

    fn get_tile(&self, col: i32, row: i32) -> Option<Tile> {
        self.tiles.get(&(col, row)).copied()
    }

    fn grid_to_world(&self, col: i32, row: i32) -> Vec2 {
        Vec2::new(col as f32 * TILE_SIZE, row as f32 * TILE_SIZE)
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut spritesheetRegistry = SpriteSheetRegistry {
        spritesheets: HashMap::new(),
    };

    let imgTerrainAndFences = asset_server.load("exteriors/1_Terrains_and_Fences_16x16.png");
    let mut layoutTerrainAndFences = TextureAtlasLayout::new_empty(UVec2::new(512, 1184));

    let imgCityTerrain = asset_server.load("exteriors/2_City_Terrains_16x16.png");
    let mut layoutCityTerrain = TextureAtlasLayout::new_empty(UVec2::new(944, 1648));

    layoutTerrainAndFences.add_texture(bevy::math::URect {
        min: UVec2::new(48, 128),
        max: UVec2::new(64, 144),
    }); // index 0
    layoutTerrainAndFences.add_texture(bevy::math::URect {
        min: UVec2::new(64, 128),
        max: UVec2::new(80, 144),
    }); // index 1
    layoutCityTerrain.add_texture(bevy::math::URect {
        min: UVec2::new(674, 464),
        max: UVec2::new(688, 480),
    }); // index 0

    let layoutHandleTerrainAndFences = atlas_layouts.add(layoutTerrainAndFences);
    spritesheetRegistry.spritesheets.insert(
        SpriteSheetKind::TerrainAndFences,
        (imgTerrainAndFences, layoutHandleTerrainAndFences),
    );
    let layoutHandleCityTerrain = atlas_layouts.add(layoutCityTerrain);
    spritesheetRegistry.spritesheets.insert(
        SpriteSheetKind::CityTerrain,
        (imgCityTerrain, layoutHandleCityTerrain),
    );
    commands.insert_resource(spritesheetRegistry);

    // let image = asset_server.load("exteriors/1_Terrains_and_Fences_16x16.png");

    // let mut layout = TextureAtlasLayout::new_empty(UVec2::new(512, 1184));

    // layout.add_texture(bevy::math::URect {
    //     min: UVec2::new(48, 128),
    //     max: UVec2::new(64, 144),
    // }); // Index 0

    // // Add dirt (16x16) at (16, 0)
    // layout.add_texture(bevy::math::URect {
    //     min: UVec2::new(464, 144),
    //     max: UVec2::new(480, 160),
    // }); // Index 1

    // let layout_handle = atlas_layouts.add(layout);

    // commands.insert_resource(TileAtlasHandle { image, layout: layout_handle });
}

fn setup_level(mut commands: Commands) {
    let mut tile_grid = TileGrid::new(20, 12);

    for col in 4..8 {
        for row in 4..8 {
            tile_grid.set_tile(col, row, Tile::grass());
        }
    }

    // Add some dirt patches
    for col in 2..4 {
        for row in 2..4 {
            tile_grid.set_tile(col, row, Tile::dirt());
        }
    }

    // Add some dirt patches
    for col in 0..2 {
        for row in 0..2 {
            tile_grid.set_tile(col, row, Tile::road());
        }
    }

    commands.insert_resource(tile_grid);
}

fn spawn_tiles(
    mut commands: Commands,
    tile_grid: Res<TileGrid>,
    spritesheetRegistry: Res<SpriteSheetRegistry>,
) {
    // let col: i32 = 0;
    // let row: i32 = 0;
    // let tile: Tile = tile_grid.tiles[col, row];
    for ((col, row), tile) in &tile_grid.tiles {
        let world_pos = tile_grid.grid_to_world(*col, *row);
        // let world_pos = tile_grid.grid_to_world(col, row);
        //
        let Some((handleImage, handleTextureAtlasLayout)) =
            spritesheetRegistry.spritesheets.get(&tile.spritesheet)
        else {
            continue;
        };

        commands.spawn((
            Sprite::from_atlas_image(
                // atlas_handle.image.clone(),
                handleImage.clone(),
                TextureAtlas {
                    // layout: atlas_handle.layout.clone(),
                    layout: handleTextureAtlasLayout.clone(),
                    index: tile.index,
                },
            ),
            Transform::from_translation(world_pos.extend(0.0)),
            bevy::sprite::Anchor::BOTTOM_LEFT,
            Name::new(format!("Tile({}, {})", col, row)),
        ));
    }
}
