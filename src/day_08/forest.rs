pub(super) struct Forest {
    pub trees: Vec<Vec<Tree>>,
    pub rows: usize,
    pub columns: usize,
}

impl Forest {
    pub fn count_trees_visible_from_outside(&self) -> u32 {
        let mut visible_tree_count = 0u32;

        for (row, treeline) in self.trees.iter().enumerate() {
            for (col, tree) in treeline.iter().enumerate() {
                if tree.is_visible_from_outside(self, row, col) {
                    visible_tree_count += 1;
                }
            }
        }

        visible_tree_count
    }

    pub fn get_highest_scenic_score(&self) -> usize {
        let mut highest_scenic_score = 0usize;

        for (row, treeline) in self.trees.iter().enumerate() {
            for (col, tree) in treeline.iter().enumerate() {
                if let Some(scenic_score) = tree.calculate_scenic_score(self, row, col) {
                    if scenic_score > highest_scenic_score {
                        highest_scenic_score = scenic_score;
                    }
                }
            }
        }

        highest_scenic_score
    }

    fn is_on_edge(&self, row: usize, col: usize) -> bool {
        row == 0 || row == self.rows - 1 || col == 0 || col == self.columns - 1
    }

    fn treelines_out_from_tree(&self, row: usize, col: usize) -> [Vec<&Tree>; 4] {
        let trees_left = (0..col)
            .rev()
            .map(|col_left| &self.trees[row][col_left])
            .collect::<Vec<&Tree>>();

        let trees_right = ((col + 1)..self.columns)
            .map(|col_right| &self.trees[row][col_right])
            .collect::<Vec<&Tree>>();

        let trees_up = (0..row)
            .rev()
            .map(|row_up| &self.trees[row_up][col])
            .collect::<Vec<&Tree>>();

        let trees_down = ((row + 1)..self.rows)
            .map(|row_down| &self.trees[row_down][col])
            .collect::<Vec<&Tree>>();

        [trees_left, trees_right, trees_up, trees_down]
    }
}

pub(super) struct Tree {
    pub height: u32,
}

impl Tree {
    fn is_visible_from_outside(
        &self,
        forest: &Forest,
        row_in_forest: usize,
        col_in_forest: usize,
    ) -> bool {
        if forest.is_on_edge(row_in_forest, col_in_forest) {
            return true;
        }

        for treeline in forest.treelines_out_from_tree(row_in_forest, col_in_forest) {
            if treeline.iter().all(|tree| tree.height < self.height) {
                return true;
            }
        }

        false
    }

    fn calculate_scenic_score(
        &self,
        forest: &Forest,
        row_in_forest: usize,
        col_in_forest: usize,
    ) -> Option<usize> {
        if forest.is_on_edge(row_in_forest, col_in_forest) {
            return None;
        }

        let mut scenic_score = 1;

        for treeline in forest.treelines_out_from_tree(row_in_forest, col_in_forest) {
            scenic_score *= self.get_viewing_distance(treeline);
        }

        Some(scenic_score)
    }

    fn get_viewing_distance(&self, trees_in_direction: Vec<&Tree>) -> usize {
        let mut viewing_distance = trees_in_direction
            .iter()
            .take_while(|tree| tree.height < self.height)
            .count();

        let total_trees = trees_in_direction.len();

        if viewing_distance < total_trees {
            viewing_distance += 1;
        }

        viewing_distance
    }
}
