use std::collections::HashMap;

use crate::day_22::position_and_direction::Position;

use super::Map;

#[derive(Clone)]
pub enum Corner {
    InnerTopLeft,
    InnerTopRight,
    InnerBottomRight,
    InnerBottomLeft,
    OuterTopLeft,
    OuterTopRight,
    OuterBottomRight,
    OuterBottomLeft,
}

impl Corner {
    pub fn is_inner(&self) -> bool {
        use Corner::*;

        matches!(
            self,
            InnerTopLeft | InnerTopRight | InnerBottomRight | InnerBottomLeft
        )
    }

    pub fn find_corners_in_map(map: &Map, cube_side_length: u32) -> HashMap<Position, Corner> {
        let cube_side_length = i64::from(cube_side_length);
        let map_rows = i64::from(map.rows);
        let map_columns = i64::from(map.columns);

        let mut corners = HashMap::<Position, Corner>::new();

        let mut row: i64 = -1;
        let mut column: i64 = -1;

        loop {
            if row > map_rows - 1 {
                break;
            }
            if column > map_columns - 1 {
                column = -1;
                row += cube_side_length;
                continue;
            }

            if let Some((position, corner_type)) = Corner::try_find_corner_in_map(map, row, column)
            {
                corners.insert(position, corner_type);
            }

            column += cube_side_length;
        }

        corners
    }

    fn try_find_corner_in_map(
        map: &Map,
        top_left_row: i64,
        top_left_column: i64,
    ) -> Option<(Position, Corner)> {
        let top_left = Position {
            row: top_left_row,
            column: top_left_column,
        };
        let top_right = Position {
            row: top_left_row,
            column: top_left_column + 1,
        };
        let bottom_right = Position {
            row: top_left_row + 1,
            column: top_left_column + 1,
        };
        let bottom_left = Position {
            row: top_left_row + 1,
            column: top_left_column,
        };

        let position_tiles = [&top_left, &top_right, &bottom_right, &bottom_left]
            .map(|position| map.get(position.row, position.column));

        use Corner::*;

        let corner = match position_tiles {
            [Some(_), Some(_), None, Some(_)] => (top_left, InnerTopLeft),
            [Some(_), Some(_), Some(_), None] => (top_right, InnerTopRight),
            [None, Some(_), Some(_), Some(_)] => (bottom_right, InnerBottomRight),
            [Some(_), None, Some(_), Some(_)] => (bottom_left, InnerBottomLeft),
            [Some(_), None, None, None] => (top_left, OuterTopLeft),
            [None, Some(_), None, None] => (top_right, OuterTopRight),
            [None, None, Some(_), None] => (bottom_right, OuterBottomRight),
            [None, None, None, Some(_)] => (bottom_left, OuterBottomLeft),
            _ => {
                return None;
            }
        };

        Some(corner)
    }
}
