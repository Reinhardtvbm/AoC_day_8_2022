use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Tree {
    height: usize,
    coordinate: Coordinate,
}

impl Tree {
    fn new(height: usize, coordinate: Coordinate) -> Self {
        Self { height, coordinate }
    }
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x: x + 1, y: y + 1 }
    }
}

struct VisibleTrees {
    vector: Vec<Tree>,
}

impl VisibleTrees {
    fn new() -> Self {
        Self { vector: Vec::new() }
    }

    fn push(&mut self, tree: &Tree) {
        if !self.vector.contains(tree) {
            self.vector.push(*tree);
        }
    }
}

fn main() {
    let file_contents = read_to_string("data.txt").unwrap();
    let lines = file_contents.lines();

    let mut forest: Vec<Vec<Tree>> = Vec::new();

    for (x, line) in lines.into_iter().enumerate() {
        let mut row: Vec<Tree> = Vec::new();

        for (y, character) in line.chars().into_iter().enumerate() {
            row.push(Tree::new(
                character.to_digit(10).unwrap() as usize,
                Coordinate::new(x, y),
            ))
        }

        println!("{:?}", row);

        forest.push(row);
    }

    let mut visible_trees = VisibleTrees::new();

    // look from right hand side
    for tree_row in &forest {
        let mut max_height = -1;

        for tree in tree_row {
            if tree.height as isize > max_height {
                visible_trees.push(tree);

                max_height = tree.height as isize;
            }

            if max_height == 9 {
                break;
            }
        }
    }

    // look from left hand side
    for tree_row in &forest {
        let mut max_height = -1;

        for tree in tree_row.into_iter().rev() {
            if tree.height as isize > max_height {
                visible_trees.push(tree);

                max_height = tree.height as isize;
            }

            if max_height == 9 {
                break;
            }
        }
    }

    // look from above
    for i in 0..forest.first().unwrap().len() {
        let mut max_height = -1;

        for j in 0..forest.len() {
            if forest[j][i].height as isize > max_height {
                visible_trees.push(&forest[j][i]);

                max_height = forest[j][i].height as isize;
            }

            if max_height == 9 {
                break;
            }
        }
    }

    // look from below
    for i in 0..forest.first().unwrap().len() {
        let mut max_height = -1;

        for j in (0..forest.len()).rev() {
            if forest[j][i].height as isize > max_height {
                visible_trees.push(&forest[j][i]);

                max_height = forest[j][i].height as isize;
            }

            if max_height == 9 {
                break;
            }
        }
    }

    let part_1 = visible_trees.vector.len();

    for tree in visible_trees.vector {
        println!("{:?}", tree);
    }

    println!("part 1: {}", part_1);

    // ================================================================================================================================
    // ================================================================================================================================
    // ================================================================================================================================

    let width = forest.first().unwrap().len();
    let height = forest.len();

    let mut scenic_scores: Vec<Vec<usize>> = vec![vec![0; width]; height];

    for i in 0..forest.len() {
        for j in 0..forest[i].len() {
            let mut right_score = 0;
            let mut left_score = 0;
            let mut up_score = 0;
            let mut down_score = 0;

            let tree_height = forest[i][j].height;

            // look right
            for k in (j + 1)..forest[i].len() {
                right_score += 1;

                if forest[i][k].height >= tree_height {
                    break;
                }
            }

            // look left
            for k in (0..=(j as isize - 1)).rev() {
                left_score += 1;

                if forest[i][k as usize].height >= tree_height {
                    break;
                }
            }

            // look up
            for k in (0..=(i as isize - 1)).rev() {
                up_score += 1;

                if forest[k as usize][j].height >= tree_height {
                    break;
                }
            }

            // look down
            for k in (i + 1)..forest.len() {
                down_score += 1;

                if forest[k][j].height >= tree_height {
                    break;
                }
            }

            scenic_scores[i][j] = right_score * left_score * up_score * down_score;
        }
    }

    let mut max_scenic_score = 0;
    let mut coord = (0, 0);
    let mut height = 0;

    for (y, row) in scenic_scores.into_iter().enumerate() {
        for (x, score) in row.into_iter().enumerate() {
            if score > max_scenic_score {
                max_scenic_score = score;
                coord = (x + 1, y + 1);
                height = forest[y][x].height;
            }
        }
    }

    println!(
        "part 2: {} at {:?} with height {}",
        max_scenic_score, coord, height
    );
}
