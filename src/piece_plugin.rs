use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_pieces);
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum PieceType {
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
    fn to_u8(value: (Self, Self)) -> u8 {
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

fn build_pieces(mut commands: Commands, mut map_query: MapQuery) {

}