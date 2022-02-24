use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BoardState>()
            .add_startup_system(create_board);
    }
}

#[derive(Component)]
pub struct Board;

#[derive(Component)]
pub struct BoardSquare {
    pos: (u8, u8),
}

pub struct BoardState {
    state: [u8; 64]
}
impl Default for BoardState {
    fn default() -> Self {
        Self {
            state: [0; 64],
        }
    }
}

fn create_board(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    let texture_handle = asset_server.load("images/chess_squares.png");

    let layer_settings = LayerSettings::new(
        MapSize(8, 8),
        ChunkSize(1, 1),
        TileSize(333.3, 334.0),
        TextureSize(1666.6, 334.0),
    );

    let (mut layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 0u16);
    for row in 0..8 {
        for col in 0..8 {
            let tile_pos = TilePos(row as u32, col as u32);
            let _ = layer_builder.set_tile(
                tile_pos,
                Tile {
                    texture_index: if (row + col + 1) % 2 == 0 { 0 } else { 1 },
                    ..Default::default()
                }.into()
            );

            if let Ok(entity) = layer_builder.get_tile_entity(&mut commands, tile_pos) {
                commands.entity(entity)
                    .insert(BoardSquare { pos: (row, col) });
            }
        }
    }
    map_query.build_layer(&mut commands, layer_builder, texture_handle);

    let texture_handle = asset_server.load("images/pieces.png");

    let layer_settings = LayerSettings::new(
        MapSize(8, 8),
        ChunkSize(1, 1),
        TileSize(333.2, 334.0),
        TextureSize(1999.2, 668.0),
    );

    let (mut layer_builder, layer_1_entity) = LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 1u16);
    layer_builder.set_tile(
        TilePos(4, 4),
        Tile {
            texture_index: 1,
            ..Default::default()
        }.into()
    );
    map_query.build_layer(&mut commands, layer_builder, texture_handle);

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    map.add_layer(&mut commands, 0u16, layer_entity);
    map.add_layer(&mut commands, 1u16, layer_1_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Board)
        // .insert(Transform::default())
        .insert(Transform {
            translation: Vec3::new(-500.0, -500.0, 0.0),
            scale: Vec3::new(0.38, 0.38, 0.0),
            ..Default::default()
        })
        .insert(GlobalTransform::default());
}