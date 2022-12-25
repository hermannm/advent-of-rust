use pathfinding::prelude::dijkstra;

pub(super) type Position = (usize, usize);

pub(super) struct HeightMap {
    pub heights: Vec<Vec<usize>>,

    pub rows: usize,
    pub columns: usize,

    pub current_position: Position,
    pub goal_position: Position,
}

impl HeightMap {
    pub fn find_shortest_path(&self) -> Result<(Vec<Position>, u32), String> {
        dijkstra(
            &self.current_position,
            |position| {
                self.get_reachable_neighbors(position)
                    .iter()
                    .map(|&neighbor| (neighbor, 1u32))
                    .collect::<Vec<(Position, u32)>>()
            },
            |&position| position == self.goal_position,
        )
        .ok_or_else(|| String::from("No shortest path found for height map"))
    }

    fn get_reachable_neighbors(&self, position: &Position) -> Vec<Position> {
        let mut reachable_neighbors = Vec::<Position>::new();

        let &(row, column) = position;

        let mut add_if_reachable = |neighbor: Position| {
            let current_position_height = self.heights[row][column];
            let neighbor_height = self.heights[neighbor.0][neighbor.1];

            if neighbor_height <= current_position_height + 1 {
                reachable_neighbors.push(neighbor);
            }
        };

        if row != 0 {
            add_if_reachable((row - 1, column));
        }
        if row < self.rows - 1 {
            add_if_reachable((row + 1, column));
        }
        if column != 0 {
            add_if_reachable((row, column - 1));
        }
        if column < self.columns - 1 {
            add_if_reachable((row, column + 1));
        }

        reachable_neighbors
    }

    pub fn positions_at_lowest_elevation(&self) -> Vec<Position> {
        let mut positions = Vec::<Position>::new();

        for (row, height_row) in self.heights.iter().enumerate() {
            for (column, height) in height_row.iter().enumerate() {
                if *height == 0 {
                    positions.push((row, column));
                }
            }
        }

        positions
    }
}
