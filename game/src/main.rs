use bevy::prelude::*;

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
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_player)
        .run();
}

fn setup(mut commands: Commands) {
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

#[derive(Component)]
struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Left,
    Down,
    Right,
}

// Character-specific spritesheet
const SPRITE_PATH: &str = "character/walk.png";
const TILE_SIZE: UVec2 = UVec2::new(16, 32); // (width, height) of one sprite
const MAX_FRAMES_PER_ANIMATION: usize = 4;
const ANIMATION_COUNT: usize = 5;
const SECONDS_PER_FRAME: f32 = 0.1;

#[derive(Component)]
struct AnimationState {
    facing: Facing,
    moving: bool,
    was_moving: bool,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(SPRITE_PATH);
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        TILE_SIZE,
        MAX_FRAMES_PER_ANIMATION as u32,
        ANIMATION_COUNT as u32,
        None,
        None,
    ));

    commands.spawn((
        Sprite::from_atlas_image(
            // use the very first sprite
            texture,
            TextureAtlas { layout, index: 0 },
        ),
        Transform::from_translation(Vec3::ZERO),
        Player,
        AnimationState {
            facing: Facing::Down,
            moving: false,
            was_moving: false,
        },
        AnimationTimer(Timer::from_seconds(SECONDS_PER_FRAME, TimerMode::Repeating)),
    ));
}
