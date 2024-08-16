use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use crate::components::*;
use crate::config::*;

pub struct MapSystemPlugin;
impl Plugin for MapSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_map);
    }
}

fn startup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let mut map = MapComponent {
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
        tiles: vec![Vec::with_capacity(MAP_HEIGHT); MAP_WIDTH], // Be careful here!
        buildings: HashMap::new(),
    };

    // Scale
    let scale_factor = 1.0 / 3.2;
    let scale_vec = Vec3::new(scale_factor, scale_factor, scale_factor);

    startup_map_tiles(&mut commands, &asset_server, &mut map, scale_vec);
    startup_map_buildings(&mut commands, &asset_server, &mut map, scale_vec);
    
    commands.spawn(map);
}

fn startup_map_tiles(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    map: &mut MapComponent,
    scale_vec: Vec3,
) {
    use TileType::*;

    // Tiles Handles
    let dirt_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("./models/dirt.glb"));
    let grass_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("./models/grass.glb"));
    let water_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("./models/water.glb"));

    // Generate Tiles
    let mut generate_tile = |i, j, tile_type| {
        let base_handle = match tile_type {
            TileType::Dirt => dirt_handle.clone(),
            TileType::Grass => grass_handle.clone(),
            TileType::Water => water_handle.clone(),
            _ => panic!("TileType not implemented"),
        };
        commands.spawn((
            SceneBundle {
                scene: base_handle,
                transform: Transform::
                    from_translation(Vec3::new(i as f32, 0.0, j as f32))
                    .with_scale(scale_vec),
                ..default()
            },
            TileComponent::new(i as i32, j as i32, tile_type),
        )).id()
    };

    macro_rules! g_tile {
        ($i:expr, $j:expr, $tile_type:expr, $expr:expr) => {
            if $expr {
                map.tiles[$i].push(
                    generate_tile($i, $j, $tile_type)
                );
                continue;
            }
        };
    }

    for i in 0..map.width {
        for j in 0..map.height {
            g_tile!(i, j, Water, 10 <= i && i <= 20 && 10 <= j && j <= 20);
            g_tile!(i, j, Grass, i % 5 <= 2 && j % 13 >= 2 && j % 13 <= 5 && (j / 13 % 2) == (i / 5 % 2));
            g_tile!(i, j, Dirt, true);
        }
    }

}

fn startup_map_buildings(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    map: &mut MapComponent,
    scale_vec: Vec3,
) {
    use BuildingType::*;

    // Buildings Handles
    let tree_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("./models/tree.glb"));
    let home_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("./models/home.glb"));
    let spring_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("./models/spring.glb"));

    // Generate Buildings
    let mut generate_building = |i, j, building_type| {
        match building_type {
            Tree => {
                return commands.spawn((
                    SceneBundle {
                        scene: tree_handle.clone(),
                        transform: Transform::
                            from_translation(Vec3::new(i as f32, 1.0, j as f32))
                            .with_scale(scale_vec),
                        ..default()
                    },
                )).id();
            },
            Home => {
                return commands.spawn((
                    SceneBundle {
                        scene: home_handle.clone(),
                        transform: Transform::
                            from_translation(Vec3::new(i as f32, 1.0, j as f32))
                            .with_scale(scale_vec),
                        ..default()
                    },
                )).id();
            },
            Spring => {
                return commands.spawn((
                    SceneBundle {
                        scene: spring_handle.clone(),
                        transform: Transform::
                            from_translation(Vec3::new(i as f32, 1.0, j as f32))
                            .with_scale(scale_vec),
                        ..default()
                    },
                )).id();
            },
            _ => panic!("BuildingType not implemented"),
        };
    };

    macro_rules! g_building {
        ($i:expr, $j:expr, $building_type:expr, $expr:expr) => {
            if $expr {
                map.buildings.insert(
                    IVec3::new($i as i32, 1, $j as i32),
                    generate_building($i, $j, $building_type)
                );
                continue;
            }
        };
    }

    for i in 0..map.width {
        for j in 0..map.height {
            g_building!(i, j, Home, i == 3 && j == 3);
            g_building!(i, j, Spring, i == 3 && j == 4);

            g_building!(i, j, Home, i == 28 && j == 28);
            g_building!(i, j, Spring, i == 28 && j == 27);

            g_building!(i, j, Tree,
                (i % 5 <= 2 && j % 13 >= 2 && j % 13 <= 5 && (j / 13 % 2) == (i / 5 % 2))
                && (i % 5 == 1 && (j / 13 % 2 == 1 && j % 13 == 3 || j / 13 % 2 == 0 && j % 13 == 4))
            );
        }
    }
}