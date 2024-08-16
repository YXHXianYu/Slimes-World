use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::config::*;

mod map;
use map::MapSystemPlugin;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            MapSystemPlugin
        );

        app.add_systems(Startup, (
            startup_camera,
            startup_lights,
            startup_slime,
        ));

        app.add_systems(Update, (
            update_tile_transform,
            update_slime_transform,
            update_camera,
        ));
    }
}

// Startup

fn startup_camera(mut commands: Commands) {
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

fn startup_lights(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::
            from_translation(Vec3::new(10.0, 10.0, 10.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn startup_slime(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_resource: Res<GameResource>,
) {
    let slime_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("./models/slime.glb"));

    let scale_factor = 1.0 / 3.2;
    let scale_vec = Vec3::new(scale_factor, scale_factor, scale_factor);

    let mut generate_slime = |x, z, id| {
        commands.spawn((
            SceneBundle {
                scene: slime_handle.clone(),
                transform: Transform::
                    from_translation(Vec3::new(x as f32, 1.0, z as f32))
                    .with_scale(scale_vec),
                ..default()
            },
            SlimeComponent {
                x,
                z,
            },
            BeControlledComponent {
                team_id: id,
            }
        ));
    };

    generate_slime(22, 24, game_resource.teams[0].id);
    generate_slime(21, 27, game_resource.teams[0].id);
    generate_slime(23, 25, game_resource.teams[0].id);
}

// Update

fn update_tile_transform(
    mut query: Query<(&mut Transform, &TileComponent)>,
) {
    query.iter_mut().for_each(|(mut transform, tile)| {
        transform.translation.x = tile.x as f32;
        transform.translation.z = tile.z as f32;
    });
}

fn update_slime_transform(
    mut query: Query<(&mut Transform, &SlimeComponent)>,
) {
    query.iter_mut().for_each(|(mut transform, slime)| {
        transform.translation.x = slime.x as f32;
        transform.translation.z = slime.z as f32;
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