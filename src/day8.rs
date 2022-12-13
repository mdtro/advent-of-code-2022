#[derive(Debug, Clone)]
struct Tree(u8);

#[derive(Debug)]
struct Map {
    trees: Vec<Tree>,
    max_row: usize,
    max_col: usize,
}

impl Map {
    fn is_visible(&self, tree_index: usize) -> (bool, usize) {
        //dbg!(&tree_index);
        let tree: &Tree = &self.trees[tree_index];
        let num_of_trees = self.trees.len();

        let column_index = tree_index % self.max_col;
        let row_index = tree_index / self.max_row;

        // [down, up, left, right]
        let mut visibility_scores: [usize; 4] = [0, 0, 0, 0];
        let mut visibility_score = 0;

        // we are at an edge; which is always visible
        if column_index == 0 // left most column
            || column_index == self.max_col - 1 // right most column
            || row_index == 0  // first row
            || row_index == self.max_row - 1
        {
            return (true, visibility_score);
        }

        // check column down
        let mut next_index = tree_index + self.max_col;
        let mut visible_down = true;
        while next_index < num_of_trees {
            visibility_scores[0] += 1;
            if self.trees[next_index].0 >= tree.0 {
                visible_down = false;
                break;
            }
            next_index += self.max_col;
        }

        // check column up
        let mut visible_up = true;
        next_index = tree_index - self.max_col;
        while next_index > 0 {
            visibility_scores[1] += 1;
            if self.trees[next_index].0 >= tree.0 {
                visible_up = false;

                break;
            }

            // next_index -= self.max_col;
            next_index = match next_index.checked_sub(self.max_col) {
                Some(i) => i,
                None => break,
            };
        }

        // check row left
        let mut visible_left = true;
        let mut left_trees = self.trees[tree_index - column_index..tree_index].to_vec();
        left_trees.reverse();

        for (_, prev_tree) in left_trees.iter().enumerate() {
            visibility_scores[2] += 1;
            if prev_tree.0 >= tree.0 {
                visible_left = false;
                break;
            }
        }

        // check row right
        let right_bound = (self.max_col - 1) - column_index;
        let mut visible_right = true;
        for (_, next_tree) in self.trees[tree_index + 1..=tree_index + right_bound]
            .iter()
            .enumerate()
        {
            visibility_scores[3] += 1;
            if next_tree.0 >= tree.0 {
                visible_right = false;
                break;
            }
        }

        visibility_score = visibility_scores.iter().product();

        // dbg!(tree_index);
        // dbg!(&visibility_scores);

        (
            visible_up || visible_down || visible_left || visible_right,
            visibility_score,
        )
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Map {
    let max_row: usize = input.lines().count();
    let mut max_col: usize = 0;

    let trees: Vec<Tree> = input
        .lines()
        .flat_map(|line| {
            if max_col == 0 {
                max_col = line.chars().count();
            }

            line.chars()
                .map(|c| Tree(c.to_digit(10).unwrap().try_into().unwrap()))
                .collect::<Vec<Tree>>()
        })
        .collect();

    Map {
        trees,
        max_row,
        max_col,
    }
}

#[aoc(day8, part1)]
fn part1(map: &Map) -> usize {
    // dbg!(&map);
    let mut visible_trees_total = 0;

    for (index, _) in map.trees.iter().enumerate() {
        if map.is_visible(index).0 {
            visible_trees_total += 1;
        }
    }

    visible_trees_total
}

#[aoc(day8, part2)]
fn part2(map: &Map) -> usize {
    let scores: Vec<usize> = map
        .trees
        .iter()
        .enumerate()
        .map(|(index, _tree)| map.is_visible(index).1)
        .collect();

    *scores.iter().max().unwrap()
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
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(21, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(8, part2(&parsed_input))
    }
}
