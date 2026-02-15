use bevy::prelude::*;
use std::collections::HashMap;

mod player;
use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // file_path: /* ENV["BEVY_ASSET_ROOT"] */,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..default()
                }),
        )
        .add_systems(Startup, setup_camera)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, (setup, setup_level, spawn_tiles).chain())
        .run();
}

fn setup_camera(mut commands: Commands) {
    simulation::print_copyright();
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::Fixed {
                width: 320.0,
                height: 180.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

const TILE_SIZE: f32 = 16.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TileKind {
    Grass,
    Dirt,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub kind: TileKind,
    pub index: usize,
    pub width: u32,
    pub height: u32,
}

impl Tile {
    fn grass() -> Self {
        Tile {
            kind: TileKind::Grass,
            index: 0,
            width: 1,
            height: 1,
        }
    }

    fn dirt() -> Self {
        Tile {
            kind: TileKind::Dirt,
            index: 1,
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
        Vec2::new(
            col as f32 * TILE_SIZE,
            row as f32 * TILE_SIZE,
        )
    }
}

#[derive(Resource)]
struct TileAtlasHandle {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load("exteriors/1_Terrains_and_Fences_16x16.png");

    let mut layout = TextureAtlasLayout::new_empty(UVec2::new(512, 1184));

    layout.add_texture(bevy::math::URect {
        min: UVec2::new(48, 128),
        max: UVec2::new(64, 144),
    }); // Index 0

    // Add dirt (16x16) at (16, 0)
    layout.add_texture(bevy::math::URect {
        min: UVec2::new(464, 144),
        max: UVec2::new(480, 160),
    }); // Index 1

    let layout_handle = atlas_layouts.add(layout);

    commands.insert_resource(TileAtlasHandle { image, layout: layout_handle });
}

fn setup_level(mut commands: Commands) {
    let mut tile_grid = TileGrid::new(20, 12);
    // let mut object_grid = ObjectGrid::new();

    // Fill background with grass
    for col in -20..20 {
        for row in -12..12 {
            tile_grid.set_tile(col, row, Tile::grass());
        }
    }

    // Add some dirt patches
    for col in -5..5 {
        for row in -4..4 {
            tile_grid.set_tile(col, row, Tile::dirt());
        }
    }

    // // Add objects (trees)
    // object_grid.add_object(7, 3, TileObject::tall_tree());
    // object_grid.add_object(12, 5, TileObject::very_tall_tree());
    // object_grid.add_object(3, 8, TileObject::tall_tree());
    // object_grid.add_object(15, 11, TileObject::tall_tree());

    commands.insert_resource(tile_grid);
    // commands.insert_resource(object_grid);
}

fn spawn_tiles(
    mut commands: Commands,
    tile_grid: Res<TileGrid>,
    atlas_handle: Res<TileAtlasHandle>,
) {
    for ((col, row), tile) in &tile_grid.tiles {
        let world_pos = tile_grid.grid_to_world(*col, *row);

        commands.spawn((
            Sprite::from_atlas_image(
                atlas_handle.image.clone(),
                TextureAtlas {
                    layout: atlas_handle.layout.clone(),
                    index: tile.index,
                },
            ),
            Transform::from_translation(world_pos.extend(0.0)),
            bevy::sprite::Anchor::BOTTOM_LEFT,
            Name::new(format!("Tile({}, {})", col, row)),
        ));
    }
}

