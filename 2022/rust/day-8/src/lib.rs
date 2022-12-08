use grid::Grid;

fn setup_grid(input: String) -> Grid<u32> {
    Grid::from_vec(
        input
            .lines()
            .flat_map(|row| {
                row.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<u32>>(),
        input.lines().next().unwrap().len(),
    )
}

fn hidden_in_row(tree: &&u32, left: &[&u32], right: &[&u32]) -> bool {
    left.iter().any(|left| left >= tree) && right.iter().any(|right| right >= tree)
}
fn hidden_in_col(tree: &&u32, up: &[&u32], down: &[&u32]) -> bool {
    up.iter().any(|up| up >= tree) && down.iter().any(|down| down >= tree)
}

pub fn part_1(input: String) -> usize {
    let grid = setup_grid(input);
    let (rows, cols) = grid.size();
    let mut visible: usize = rows * cols;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            let tree = grid.get(row, col).unwrap();

            let current_row = grid.iter_row(row).collect::<Vec<&u32>>();
            let current_col = grid.iter_col(col).collect::<Vec<&u32>>();

            let (left, right) = current_row.split_at(col);
            let (up, down) = current_col.split_at(row);

            if hidden_in_row(&tree, left, right.split_first().unwrap().1)
                && hidden_in_col(&tree, up, down.split_first().unwrap().1)
            {
                visible -= 1;
            }
        }
    }

    visible
}

fn visible_trees(tree: &&u32, trees: &[&u32]) -> usize {
    match trees
        .iter()
        .enumerate()
        .find(|(_, visible)| *visible >= tree)
    {
        Some((idx, _)) => idx + 1,
        None => trees.iter().len(),
    }
}

pub fn part_2(input: String) -> usize {
    let grid = setup_grid(input);
    let (rows, cols) = grid.size();
    let mut score: usize = 0;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            let tree = grid.get(row, col).unwrap();

            let mut current_row = grid.iter_row(row).collect::<Vec<&u32>>();
            let mut current_col = grid.iter_col(col).collect::<Vec<&u32>>();

            let (left, right) = current_row.split_at_mut(col);
            let (up, down) = current_col.split_at_mut(row);

            (*left).reverse();
            (*up).reverse();

            let up_score = visible_trees(&tree, up);
            let down_score = visible_trees(&tree, down.split_first().unwrap().1);
            let left_score = visible_trees(&tree, left);
            let right_score = visible_trees(&tree, right.split_first().unwrap().1);
            let total_score = up_score * down_score * left_score * right_score;
            if total_score > score {
                score = total_score
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn visible_trees() {
        assert_eq!(part_1(INPUT.to_string()), 21);
    }

    #[test]
    fn best_tree_view_score() {
        assert_eq!(part_2(INPUT.to_string()), 8);
    }
}
