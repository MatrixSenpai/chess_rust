use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::piece_plugin::PieceType;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BoardState>()
            .init_resource::<MoveList>()
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
    pub updated: bool,
    state: [u8; 64],
}
impl Default for BoardState {
    fn default() -> Self {
        Self {
            updated: false,
            state: [0; 64],
        }
    }
}
impl BoardState {
    pub fn set_state(&mut self, x: usize, y: usize, piece: u8) {
        self.state[x * 8 + y] = piece;
        self.updated = true;
    }
    pub fn get_state(&self, x: usize, y: usize) -> u8 {
        self.state[x * 8 + y]
    }
    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.state[x * 8 + y] == 0
    }
}

#[derive(Default)]
pub struct MoveList {
    moves: Vec<(u16, u16)>,
}
impl MoveList {
    pub fn get_last_move_position(&self) -> u16 {
        if self.moves.len() == 0 { 0 }
        else { self.moves.last().unwrap().1 }
    }
}

fn create_board(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    let texture_handle = asset_server.load("images/chess_square.png");

    let layer_settings = LayerSettings::new(
        MapSize(8, 8),
        ChunkSize(1, 1),
        TileSize(16.0, 16.0),
        TextureSize(80.0, 16.0)
    );

    let (mut layer_builder, board_layer_entity) = LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 0u16);
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
    commands.entity(board_layer_entity)
        .insert(Transform {
            translation: Vec3::new(-510.0, -510.0, 0.0),
            scale: Vec3::new(8.0, 8.0, 0.0),
            ..Default::default()
        });

    let texture_handle = asset_server.load("images/pieces.png");

    let layer_settings = LayerSettings::new(
        MapSize(8, 8),
        ChunkSize(1, 1),
        TileSize(320.0, 320.0),
        TextureSize(1920.0, 640.0),
    );

    let (layer_builder, piece_layer_entity) = LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 1u16);
    map_query.build_layer(&mut commands, layer_builder, texture_handle);
    commands.entity(piece_layer_entity)
        .insert(Transform {
            translation: Vec3::new(-510.0, -510.0, 0.0),
            scale: Vec3::new(0.40, 0.40, 0.0),
            ..Default::default()
        });

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    map.add_layer(&mut commands, 0u16, board_layer_entity);
    map.add_layer(&mut commands, 1u16, piece_layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Board)
        .insert(Transform::default())
        .insert(GlobalTransform::default());
}