use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
// use crate::utils::*;
use crate::config::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Startup
        app.add_systems(Startup, setup_map);
        app.add_systems(Startup, setup_camera);
        app.add_systems(Startup, setup_lights);

        // Update
        app.add_systems(Update, update_tile_transform);
    }
}

// Startup

fn setup_map(
    mut commands: Commands,
    map: Res<MapResource>,
    asset_server: Res<AssetServer>,
) {

    // Model Handles
    let dirt_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("./models/dirt.glb"));
    let grass_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("./models/grass.glb"));
    let water_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("./models/water.glb"));

    // Spawn
    let scale_factor = 1.0 / 3.2;
    let scale_vec = Vec3::new(scale_factor, scale_factor, scale_factor);

    let mut generate_tile = |i, j, tile_type| {
        let handle = match tile_type {
            TileType::Dirt => dirt_handle.clone(),
            TileType::Grass => grass_handle.clone(),
            TileType::Water => water_handle.clone(),
            _ => panic!("TileType not implemented"),
        };
        commands.spawn((
            SceneBundle {
                scene: handle,
                transform: Transform::
                    from_translation(Vec3::new(i as f32, 0.0, j as f32))
                    .with_scale(scale_vec),
                ..default()
            },
            TileComponent::new(i as i32, j as i32, tile_type),
        ));
    };

    for i in 0..map.width {
        for j in 0..map.height {

            if 10 <= i && i <= 20 && 10 <= j && j <= 20 {
                generate_tile(i, j, TileType::Water);
                continue;
            }

            if i % 5 <= 2 && j % 13 >= 2 && j % 13 <= 5 && (j / 13 % 2) == (i / 5 % 2) {
                generate_tile(i, j, TileType::Grass);
                continue;
            }

            generate_tile(i, j, TileType::Dirt);
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let position = Vec3::new(CAMERA_POS_X, CAMERA_POS_Y, CAMERA_POS_Z);
    let look_at = Vec3::new(CAMERA_LOOK_AT_X, CAMERA_LOOK_AT_Y, CAMERA_LOOK_AT_Z);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(
            position
        ).looking_at(
            look_at,
            Vec3::Y
        ),
        ..Default::default()
    });
}

fn setup_lights(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
        ..default()
    });
}

// Update

fn update_tile_transform(
    mut query: Query<(&mut Transform, &TileComponent)>,
) {
    query.iter_mut().for_each(|(mut transform, tile)| {
        transform.translation.x = tile.x as f32;
        transform.translation.z = tile.y as f32;
    });
}