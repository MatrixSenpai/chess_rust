use std::convert::From;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::board_plugin::BoardState;

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_pieces)
            .add_system(place_pieces);
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
    King = 0x00,
    Queen = 0x01,
    Bishop = 0x02,
    Knight = 0x03,
    Rook = 0x04,
    Pawn = 0x05,

    White = 0x08,
    Black = 0x10,
}
impl PieceType {
    fn from_u8(value: u8) -> (Self, Self) {
        let p_type = value & 0x07;
        let p_color = value & 0x18;

        let p_type = match p_type {
            0x00 => Self::King,
            0x01 => Self::Queen,
            0x02 => Self::Bishop,
            0x03 => Self::Knight,
            0x04 => Self::Rook,
            0x05 => Self::Pawn,
            _ => unreachable!()
        };
        let p_color = match p_color {
            0x08 => Self::White,
            0x10 => Self::Black,
            _ => unreachable!()
        };

        (p_type, p_color)
    }
    pub(crate) fn to_u8(value: (Self, Self)) -> u8 {
        let p_type = match value.0 {
            Self::King => 0x00,
            Self::Queen => 0x01,
            Self::Bishop => 0x02,
            Self::Knight => 0x03,
            Self::Rook => 0x04,
            Self::Pawn => 0x05,
            _ => unreachable!()
        };
        let p_color = match value.1 {
            Self::White => 0x08,
            Self::Black => 0x10,
            _ => unreachable!()
        };

        p_type | p_color
    }
}

#[derive(Component)]
struct Piece(u8);

fn build_pieces(mut commands: Commands, mut board_state: ResMut<BoardState>) {
    board_state.set_state(0, 0, PieceType::to_u8((PieceType::Rook, PieceType::White)));
    board_state.set_state(1, 0, PieceType::to_u8((PieceType::Knight, PieceType::White)));
    board_state.set_state(2, 0, PieceType::to_u8((PieceType::Bishop, PieceType::White)));
    board_state.set_state(3, 0, PieceType::to_u8((PieceType::King, PieceType::White)));
    board_state.set_state(4, 0, PieceType::to_u8((PieceType::Queen, PieceType::White)));
    board_state.set_state(5, 0, PieceType::to_u8((PieceType::Bishop, PieceType::White)));
    board_state.set_state(6, 0, PieceType::to_u8((PieceType::Knight, PieceType::White)));
    board_state.set_state(7, 0, PieceType::to_u8((PieceType::Rook, PieceType::White)));

    for i in 0..8 {
        board_state.set_state(i, 1, PieceType::to_u8((PieceType::Pawn, PieceType::White)));
    }

    board_state.set_state(0, 7, PieceType::to_u8((PieceType::Rook, PieceType::Black)));
    board_state.set_state(1, 7, PieceType::to_u8((PieceType::Knight, PieceType::Black)));
    board_state.set_state(2, 7, PieceType::to_u8((PieceType::Bishop, PieceType::Black)));
    board_state.set_state(3, 7, PieceType::to_u8((PieceType::King, PieceType::Black)));
    board_state.set_state(4, 7, PieceType::to_u8((PieceType::Queen, PieceType::Black)));
    board_state.set_state(5, 7, PieceType::to_u8((PieceType::Bishop, PieceType::Black)));
    board_state.set_state(6, 7, PieceType::to_u8((PieceType::Knight, PieceType::Black)));
    board_state.set_state(7, 7, PieceType::to_u8((PieceType::Rook, PieceType::Black)));

    for i in 0..8 {
        board_state.set_state(i, 6, PieceType::to_u8((PieceType::Pawn, PieceType::Black)));
    }
}

fn place_pieces(mut commands: Commands, mut map_query: MapQuery, board_state: Res<BoardState>) {
    map_query.despawn_layer_tiles(&mut commands, 0u16, 1u16);

    for row in 0..8 {
        for col in 0..8 {
            if board_state.is_empty(row, col) { continue }

            let pos = TilePos(row as u32, col as u32);
            let piece = board_state.get_state(row, col);
            let pc = piece & 0x18;
            let pt = piece & 0x07;

            let texture_index = pt + (pc - 8);
            println!("piece {:b} pc {:b} pt {:b} texture {}", piece, pc, pt, texture_index);
            let tile_result = map_query.set_tile(
                &mut commands,
                pos,
                Tile { texture_index: texture_index as u16, ..Default::default() },
                0u16, 1u16
            );

            if let Ok(entity) = tile_result {
                commands.entity(entity)
                    .insert(Piece(piece));
            }
        }
    }
}