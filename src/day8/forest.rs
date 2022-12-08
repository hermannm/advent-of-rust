pub struct Forest {
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

    fn trees_left<'a>(&'a self, row: &'a usize, col: &'a usize) -> impl TreeIterator<'a> {
        (0..*col).rev().map(|col_left| &self.trees[*row][col_left])
    }

    fn trees_right<'a>(&'a self, row: &'a usize, col: &'a usize) -> impl TreeIterator<'a> {
        ((*col + 1)..self.columns).map(|col_right| &self.trees[*row][col_right])
    }

    fn trees_up<'a>(&'a self, row: &'a usize, col: &'a usize) -> impl TreeIterator<'a> {
        (0..*row).rev().map(|row_up| &self.trees[row_up][*col])
    }

    fn trees_down<'a>(&'a self, row: &'a usize, col: &'a usize) -> impl TreeIterator<'a> {
        ((*row + 1)..self.rows).map(|row_down| &self.trees[row_down][*col])
    }
}

pub struct Tree {
    pub height: u32,
}

impl Tree {
    fn calculate_scenic_score(
        &self,
        forest: &Forest,
        row_in_forest: usize,
        col_in_forest: usize,
    ) -> Option<usize> {
        if forest.is_on_edge(row_in_forest, col_in_forest) {
            return None;
        }

        let visible_trees_left = forest
            .trees_left(&row_in_forest, &col_in_forest)
            .count_trees_visible_from(self);

        let visible_trees_right = forest
            .trees_right(&row_in_forest, &col_in_forest)
            .count_trees_visible_from(self);

        let visible_trees_up = forest
            .trees_up(&row_in_forest, &col_in_forest)
            .count_trees_visible_from(self);

        let visible_trees_down = forest
            .trees_down(&row_in_forest, &col_in_forest)
            .count_trees_visible_from(self);

        Some(visible_trees_left * visible_trees_right * visible_trees_up * visible_trees_down)
    }

    fn is_visible_from_outside(
        &self,
        forest: &Forest,
        row_in_forest: usize,
        col_in_forest: usize,
    ) -> bool {
        if forest.is_on_edge(row_in_forest, col_in_forest) {
            return true;
        }

        let visible_from_left = forest
            .trees_left(&row_in_forest, &col_in_forest)
            .all_shorter_than(self);

        let visible_from_right = forest
            .trees_right(&row_in_forest, &col_in_forest)
            .all_shorter_than(self);

        let visible_from_top = forest
            .trees_up(&row_in_forest, &col_in_forest)
            .all_shorter_than(self);

        let visible_from_bottom = forest
            .trees_down(&row_in_forest, &col_in_forest)
            .all_shorter_than(self);

        visible_from_left || visible_from_right || visible_from_top || visible_from_bottom
    }
}

pub trait TreeIterator<'a>: Iterator<Item = &'a Tree> + Clone {
    fn all_shorter_than(&mut self, tree: &Tree) -> bool;

    fn count_trees_visible_from(self, tree: &Tree) -> usize;
}

impl<'a, T: Iterator<Item = &'a Tree> + Clone> TreeIterator<'a> for T {
    fn all_shorter_than(&mut self, tree: &Tree) -> bool {
        self.all(|other_tree| other_tree.height < tree.height)
    }

    fn count_trees_visible_from(self, tree: &Tree) -> usize {
        let mut visible_trees = self
            .clone()
            .take_while(|other_tree| other_tree.height < tree.height)
            .count();

        let total_trees = self.count();

        if visible_trees < total_trees {
            visible_trees += 1;
        }

        visible_trees
    }
}
