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
        .add_systems(Update, (move_player, animate_player))
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
const MOVE_SPEED: f32 = 140.0; // pixels per second
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

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    let Ok((mut transform, mut anim)) = player.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();
        transform.translation.x = (transform.translation.x + delta.x).round();
        transform.translation.y = (transform.translation.y + delta.y).round();
        anim.moving = true;

        // Update facing based on dominant direction
        if direction.x.abs() > direction.y.abs() {
            anim.facing = if direction.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            };
        } else {
            anim.facing = if direction.y > 0.0 {
                Facing::Up
            } else {
                Facing::Down
            };
        }

        transform.scale.x = if anim.facing == Facing::Left { -1.0 } else { 1.0 };
    } else {
        anim.moving = false;
    }
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    let Ok((mut anim, mut timer, mut sprite)) = query.single_mut() else {
        return;
    };

    let atlas = match sprite.texture_atlas.as_mut() {
        Some(a) => a,
        None => return,
    };

    // Compute the target row and current position in the atlas
    let target_row = row_zero_based(anim.facing);
    let mut current_col = atlas.index % MAX_FRAMES_PER_ANIMATION;
    let mut current_row = atlas.index / MAX_FRAMES_PER_ANIMATION;

    // If the facing changed (or we weren't on a walking row), snap to the first frame of the target row
    if current_row != target_row {
        atlas.index = row_start_index(anim.facing);
        current_col = 0;
        current_row = target_row;
        timer.reset();
    }

    let just_started = anim.moving && !anim.was_moving;
    let just_stopped = !anim.moving && anim.was_moving;

    if anim.moving {
        if just_started {
            // On tap or movement start, immediately advance one frame for visible feedback
            let row_start = row_start_index(anim.facing);
            let next_col = (current_col + 1) % MAX_FRAMES_PER_ANIMATION;
            atlas.index = row_start + next_col;
            // Restart the timer so the next advance uses a full interval
            timer.reset();
        } else {
            // Continuous movement: advance based on timer cadence
            timer.tick(time.delta());
            if timer.just_finished() {
                let row_start = row_start_index(anim.facing);
                let next_col = (current_col + 1) % MAX_FRAMES_PER_ANIMATION;
                atlas.index = row_start + next_col;
            }
        }
    } else if just_stopped {
        // Not moving: keep current frame to avoid snap. Reset timer on transition to idle.
        timer.reset();
    }

    // Update previous movement state
    anim.was_moving = anim.moving;
}

fn row_start_index(facing: Facing) -> usize {
    row_zero_based(facing) * MAX_FRAMES_PER_ANIMATION
}

fn row_zero_based(facing: Facing) -> usize {
    match facing {
        Facing::Up => 4,
        Facing::Left => 2,
        Facing::Down => 0,
        Facing::Right => 2,
    }
}
