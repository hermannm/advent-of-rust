use std::collections::HashMap;

use self::{
    corners::Corner,
    edges::{Edge, EdgeFromCorner},
};

use super::position_and_direction::{Direction, Position};

mod corners;
mod edges;

pub(super) struct Map {
    tiles: Vec<Vec<Option<Tile>>>,
    rows: u32,
    columns: u32,
    cube_adjacency_map: Option<AdjacencyMap>,
}

#[derive(Clone)]
pub(super) enum Tile {
    Open,
    Wall,
}

pub(super) type AdjacencyMap = HashMap<(Position, Direction), (Position, Direction)>;

impl Map {
    pub fn new(tiles: Vec<Vec<Option<Tile>>>, rows: u32, columns: u32) -> Self {
        Self {
            tiles,
            rows,
            columns,
            cube_adjacency_map: None,
        }
    }

    fn get(&self, row: i64, column: i64) -> Option<Tile> {
        let row_index = usize::try_from(row).ok()?;
        let column_index = usize::try_from(column).ok()?;

        self.tiles.get(row_index)?.get(column_index)?.to_owned()
    }

    pub fn top_left_position(&self) -> Result<Position, String> {
        for (row_index, rows) in self.tiles.iter().enumerate() {
            for (column_index, tile) in rows.iter().enumerate() {
                if let Some(Tile::Open) = tile {
                    let row = i64::try_from(row_index).map_err(|_| {
                        format!("Failed to convert index '{row_index}' to signed integer")
                    })?;

                    let column = i64::try_from(column_index).map_err(|_| {
                        format!("Failed to convert index '{column_index}' to signed integer")
                    })?;

                    return Ok(Position { row, column });
                }
            }
        }

        Err(String::from("Failed to find top-left position in map"))
    }

    pub fn next_tile_in_direction(
        &self,
        current_position: &Position,
        direction: &Direction,
    ) -> Result<(Tile, Position, Direction), String> {
        let next_position = current_position.other_position_in_direction(direction, 1);

        if let Some(tile) = self.get(next_position.row, next_position.column) {
            return Ok((tile, next_position, direction.clone()));
        }

        match &self.cube_adjacency_map {
            None => {
                let (tile, next_position) = self.wrap_flat(current_position, direction);
                Ok((tile, next_position, direction.clone()))
            }
            Some(adjacency_map) => self.wrap_cube(current_position, direction, adjacency_map),
        }
    }

    fn wrap_flat(&self, current_position: &Position, direction: &Direction) -> (Tile, Position) {
        let mut next_position = current_position.clone();

        let mut tile_on_other_side = self
            .get(current_position.row, current_position.column)
            .expect("Current position should always be an open tile");

        let opposite_direction = direction.opposite();

        loop {
            let neighbor = next_position.other_position_in_direction(&opposite_direction, 1);

            if let Some(tile) = self.get(neighbor.row, neighbor.column) {
                tile_on_other_side = tile;
                next_position = neighbor;
            } else {
                break;
            }
        }

        (tile_on_other_side, next_position)
    }

    fn wrap_cube(
        &self,
        current_position: &Position,
        current_direction: &Direction,
        adjacency_map: &AdjacencyMap,
    ) -> Result<(Tile, Position, Direction), String> {
        let (new_position, new_direction) = adjacency_map
            .get(&(current_position.clone(), current_direction.clone()))
            .ok_or_else(|| {
                format!(
                    "Failed to find entry for position {},{} and direction {:?} in adjacency map",
                    current_position.row, current_position.column, current_direction
                )
            })?;

        let tile = self
            .get(new_position.row, new_position.column)
            .ok_or_else(|| {
                format!(
                    "Expected wrapped position {},{} to be a tile, but it was not",
                    new_position.row, new_position.column
                )
            })?;

        Ok((tile, new_position.clone(), new_direction.clone()))
    }

    pub fn set_to_cube(&mut self, cube_side_length: u32) {
        let mut adjacency_map = HashMap::<(Position, Direction), (Position, Direction)>::new();

        let corners = Corner::find_corners_in_map(self, cube_side_length);

        let inner_corners = corners
            .iter()
            .filter_map(|(position, corner_type)| {
                if corner_type.is_inner() {
                    Some((position.clone(), corner_type.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<(Position, Corner)>>();

        'inner_corner_loop: for (inner_corner_position, inner_corner_type) in inner_corners {
            let mut left_edge = Edge::new(
                inner_corner_type.clone(),
                EdgeFromCorner::Left,
                &inner_corner_position,
                cube_side_length,
            );

            let mut right_edge = Edge::new(
                inner_corner_type,
                EdgeFromCorner::Right,
                &inner_corner_position,
                cube_side_length,
            );

            loop {
                let Some(next_corner_left) = left_edge.next_outer_corner(&corners) else {
                    continue 'inner_corner_loop;
                };
                let Some(next_corner_right) = right_edge.next_outer_corner(&corners) else {
                    continue 'inner_corner_loop;
                };

                Edge::add_adjacency(&left_edge, &right_edge, &mut adjacency_map);

                left_edge.replace_with_next_edge(next_corner_left);
                right_edge.replace_with_next_edge(next_corner_right);
            }
        }

        self.cube_adjacency_map = Some(adjacency_map);
    }
}
