use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
// use crate::utils::*;
use crate::config::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_map,
            setup_camera,
            setup_lights
        ));

        app.add_systems(Update, (
            update_tile_transform,
            update_camera
        ));
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
    let tree_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("./models/tree.glb"));

    // Spawn
    let scale_factor = 1.0 / 3.2;
    let scale_vec = Vec3::new(scale_factor, scale_factor, scale_factor);

    let mut generate_tile = |i, j, tile_type| {
        let base_handle = match tile_type {
            TileType::Dirt => dirt_handle.clone(),
            TileType::Grass => grass_handle.clone(),
            TileType::Water => water_handle.clone(),
            TileType::Tree => grass_handle.clone(),
            _ => panic!("TileType not implemented"),
        };

        // Base
        let entity = commands.spawn((
            SceneBundle {
                scene: base_handle,
                transform: Transform::
                    from_translation(Vec3::new(i as f32, 0.0, j as f32))
                    .with_scale(scale_vec),
                ..default()
            },
            TileComponent::new(i as i32, j as i32, tile_type),
        )).id();

        // Sub (e.g. Tree, Building)
        match tile_type {
            TileType::Tree => {
                let tree_entity = commands.spawn((
                    SceneBundle {
                        scene: tree_handle.clone(),
                        transform: Transform::
                            from_translation(Vec3::new(i as f32, 1.0, j as f32))
                            .with_scale(scale_vec),
                        ..default()
                    },
                )).id();
                commands.entity(entity).insert(SubEntitiesComponent{
                    self_entity: entity,
                    sub_entities: vec![tree_entity],
                });
            },
            _ => (),
        };
    };

    for i in 0..map.width {
        for j in 0..map.height {

            if 10 <= i && i <= 20 && 10 <= j && j <= 20 {
                generate_tile(i, j, TileType::Water);
                continue;
            }

            if i % 5 <= 2 && j % 13 >= 2 && j % 13 <= 5 && (j / 13 % 2) == (i / 5 % 2) {
                if i % 5 == 1 && (j / 13 % 2 == 1 && j % 13 == 3 || j / 13 % 2 == 0 && j % 13 == 4) {
                    generate_tile(i, j, TileType::Tree);
                    continue;
                }
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

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(
                position
            ).looking_at(
                look_at,
                Vec3::Y
            ),
            ..Default::default()
        },
    ));
}

fn setup_lights(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::
            from_translation(Vec3::new(100.0, 100.0, 100.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
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

fn update_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    kb: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut cam = camera.single_mut();

    // keyboard
    let f = |keycode| {
        kb.pressed(keycode) as i32 as f32
    };
    let x = f(KeyCode::KeyD) - f(KeyCode::KeyA);
    let y = f(KeyCode::KeyQ) - f(KeyCode::KeyE);
    let z = f(KeyCode::KeyW) - f(KeyCode::KeyS); // forward is negative z
    
    let speed = time.delta_seconds() * CAMERA_MOVE_SPEED;
    if x != 0.0 {
        let right = cam.right();
        cam.translation += right * x * speed;
    }
    if y != 0.0 {
        cam.translation += Vec3::Y * y * speed;
    }
    if z != 0.0 {
        let forward = cam.forward();
        cam.translation += forward * z * speed;
    }

    // mouse
    if mouse.pressed(MouseButton::Right) {
        let mut delta = Vec2::ZERO;
        for event in mouse_motion.read() {
            delta += event.delta;
        }
        let delta = delta * CAMERA_ROTATE_SPEED;

        cam.rotate_y(-delta.x);
        cam.rotate_local_x(-delta.y);
    }

}