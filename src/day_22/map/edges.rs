use std::collections::HashMap;

use crate::day_22::position_and_direction::{Direction, Position};

use super::{corners::Corner, AdjacencyMap};

pub(super) struct Edge {
    from_corner: Corner,
    direction_from_corner: EdgeFromCorner,
    direction: Direction,
    start_position: Position,
    length: i64,
}

pub(super) enum EdgeFromCorner {
    Left,
    Right,
}

pub(super) struct EdgeCorner {
    pub corner_type: Corner,
    position: Position,
    is_on_edge: bool,
}

impl Edge {
    pub fn new(
        from_corner: Corner,
        direction_from_corner: EdgeFromCorner,
        corner_position: &Position,
        edge_length: u32,
    ) -> Self {
        let direction = Edge::direction(&from_corner, &direction_from_corner);

        let start_position =
            Edge::start_position_from_corner(corner_position, &from_corner, &direction);

        Self {
            from_corner,
            direction_from_corner,
            direction,
            start_position,
            length: i64::from(edge_length),
        }
    }

    fn start_position_from_corner(
        corner_position: &Position,
        corner_type: &Corner,
        edge_direction: &Direction,
    ) -> Position {
        if corner_type.is_inner() {
            corner_position.other_position_in_direction(edge_direction, 1)
        } else {
            corner_position.clone()
        }
    }

    fn direction(from_corner: &Corner, direction_from_corner: &EdgeFromCorner) -> Direction {
        use Corner::*;
        use Direction::*;
        use EdgeFromCorner::*;

        match (&from_corner, &direction_from_corner) {
            (InnerTopLeft, Left) => East,
            (InnerTopLeft, Right) => South,
            (InnerTopRight, Left) => South,
            (InnerTopRight, Right) => West,
            (InnerBottomRight, Left) => West,
            (InnerBottomRight, Right) => North,
            (InnerBottomLeft, Left) => North,
            (InnerBottomLeft, Right) => East,
            (OuterTopLeft, Left) => North,
            (OuterTopLeft, Right) => West,
            (OuterTopRight, Left) => East,
            (OuterTopRight, Right) => North,
            (OuterBottomRight, Left) => South,
            (OuterBottomRight, Right) => East,
            (OuterBottomLeft, Left) => West,
            (OuterBottomLeft, Right) => South,
        }
    }

    pub fn next_outer_corner(&self, corners: &HashMap<Position, Corner>) -> Option<EdgeCorner> {
        let mut corner_position = self.start_position.clone();

        let mut is_on_edge = true;

        loop {
            let steps = if is_on_edge {
                self.length - 1
            } else {
                self.length
            };

            corner_position = corner_position.other_position_in_direction(&self.direction, steps);

            if let Some(corner_type) = corners.get(&corner_position) {
                return Some(EdgeCorner {
                    corner_type: corner_type.clone(),
                    position: corner_position,
                    is_on_edge,
                });
            } else {
                let potential_inner_corner_position =
                    corner_position.other_position_in_direction(&self.direction, 1);

                if let Some(corner_type) = corners.get(&potential_inner_corner_position) {
                    if corner_type.is_inner() {
                        return None;
                    }
                }
            }

            is_on_edge = false;
        }
    }

    pub fn replace_with_next_edge(&mut self, next_corner: EdgeCorner) {
        if next_corner.is_on_edge {
            let direction = Edge::direction(&next_corner.corner_type, &self.direction_from_corner);

            self.start_position = Edge::start_position_from_corner(
                &next_corner.position,
                &next_corner.corner_type,
                &direction,
            );
            self.direction = direction;
            self.from_corner = next_corner.corner_type;
        } else {
            self.start_position = self
                .start_position
                .other_position_in_direction(&self.direction, self.length);
        }
    }

    pub fn add_adjacency(left_edge: &Edge, right_edge: &Edge, adjacency_map: &mut AdjacencyMap) {
        let mut direction_from_right_edge = right_edge.direction.clone();
        direction_from_right_edge.turn_left();

        let mut direction_to_right_edge = right_edge.direction.clone();
        direction_to_right_edge.turn_right();

        let mut direction_from_left_edge = left_edge.direction.clone();
        direction_from_left_edge.turn_right();

        let mut direction_to_left_edge = left_edge.direction.clone();
        direction_to_left_edge.turn_left();

        let right_edge_positions = right_edge.positions();
        let left_edge_positions = left_edge.positions();

        for (index, right_position) in right_edge_positions.iter().enumerate() {
            let left_position = &left_edge_positions[index];

            adjacency_map.insert(
                (left_position.clone(), direction_from_left_edge.clone()),
                (right_position.clone(), direction_to_right_edge.clone()),
            );

            adjacency_map.insert(
                (right_position.clone(), direction_from_right_edge.clone()),
                (left_position.clone(), direction_to_left_edge.clone()),
            );
        }
    }

    fn positions(&self) -> Vec<Position> {
        let mut positions = Vec::<Position>::new();
        let mut length_from_start_position: i64 = 0;

        while length_from_start_position < self.length {
            let position = self
                .start_position
                .other_position_in_direction(&self.direction, length_from_start_position);

            positions.push(position);

            length_from_start_position += 1;
        }

        positions
    }
}
