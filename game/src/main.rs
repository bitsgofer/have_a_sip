use bevy::prelude::*;

mod player;
use crate::player::PlayerPlugin;
mod map;
use crate::map::MapPlugin;

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
        .add_plugins(MapPlugin)
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
