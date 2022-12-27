use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<i8>>,
}

impl Map {
    fn add_row(&mut self, row: Vec<i8>) {
        self.grid.push(row);
    }

    fn num_cols(&self) -> i64 {
        self.grid[0].len() as i64
    }

    fn num_rows(&self) -> i64 {
        self.grid.len() as i64
    }

    fn calculate_trees_from_edges(&self) -> i64 {
        (self.num_rows() * 2) + (self.num_cols() * 2) - 4
    }

    fn calculate_interior_trees(&self) -> i64 {
        let starting_x = 1;
        let starting_y = 1;
        let ending_x = self.num_rows() - 1;
        let ending_y = self.num_cols() - 1;
        let mut visible_trees = 0;
        for x in starting_x..ending_x {
            for y in starting_y..ending_y {
                if self.check_tree_visible(x, y) {
                    visible_trees += 1;
                }
            }
        }

        return visible_trees
    }

    fn check_tree_visible(&self, x: i64 ,y: i64) -> bool {
        self.check_visible_from_top(x, y) ||
        self.check_visible_from_right(x, y) ||
        self.check_visible_from_bottom(x, y) ||
        self.check_visible_from_left(x, y)
    }

    fn check_visible_from_top(&self, x: i64, y: i64) -> bool {
        let mut x = x as usize;
        let y = y as usize;
        let tree_height = self.grid[x][y];
        while x > 0 {
            if self.grid[x - 1][y] >= tree_height {
                return false;
            }
            x -= 1;
        }
        return true
    }

    fn check_visible_from_right(&self, x: i64, y: i64) -> bool {
        let x = x as usize;
        let mut y = y as usize;
        let tree_height = self.grid[x][y];
        while y < self.num_cols() as usize - 1 {
            if self.grid[x][y + 1] >= tree_height {
                return false;
            }
            y += 1;
        }
        return true
    }

    fn check_visible_from_bottom(&self, x: i64, y: i64) -> bool {
        let mut x = x as usize;
        let y = y as usize;
        let tree_height = self.grid[x][y];
        while x < self.num_rows() as usize - 1 {
            if self.grid[x + 1][y] >= tree_height {
                return false;
            }
            x += 1;
        }
        true
    }

    fn check_visible_from_left(&self, x: i64, y: i64) -> bool {
        let x = x as usize;
        let mut y = y as usize;
        let tree_height = self.grid[x][y];
        while y > 0 {
            if self.grid[x][y - 1] >= tree_height {
                return false;
            }
            y -= 1;
        }
        true
    }

    fn total_visible_trees(&self) -> i64 {
        self.calculate_trees_from_edges() + self.calculate_interior_trees()
    }

    fn visible_trees_from_top(&self, x: i64, y: i64) -> i64 {
        let mut x = x as usize;
        let y = y as usize;
        let tree_height = self.grid[x][y];
        let mut visible_trees = 0;
        while x > 0 {
            visible_trees += 1;
            if self.grid[x - 1][y] >= tree_height {
                break;
            }
            x -= 1;
        }

        return visible_trees;
    }

    fn visible_trees_from_right(&self, x: i64, y: i64) -> i64 {
        let x = x as usize;
        let mut y = y as usize;
        let tree_height = self.grid[x][y];
        let mut visible_trees = 0;
        while y < self.num_cols() as usize - 1 {
            visible_trees += 1;
            if self.grid[x][y + 1] >= tree_height {
                break;
            }
            y += 1;
        }

        return visible_trees;
    }

    fn visible_trees_from_bottom(&self, x: i64, y: i64) -> i64 {
        let mut x = x as usize;
        let y = y as usize;
        let tree_height = self.grid[x][y];
        let mut visible_trees = 0;
        while x < self.num_rows() as usize - 1 {
            visible_trees += 1;
            if self.grid[x + 1][y] >= tree_height {
                break;
            }
            x += 1;
        }

        return visible_trees;
    }

    fn visible_trees_from_left(&self, x: i64, y: i64) -> i64 {
        let x = x as usize;
        let mut y = y as usize;
        let tree_height = self.grid[x][y];
        let mut visible_trees = 0;
        while y > 0 {
            visible_trees += 1;
            if self.grid[x][y - 1] >= tree_height {
                break;
            }
            y -= 1;
        }

        return visible_trees;
    }

    fn find_highest_tree_score(&self) -> i64 {
        let starting_x = 1;
        let starting_y = 1;
        let ending_x = self.num_rows() - 1;
        let ending_y = self.num_cols() - 1;
        let mut highest_tree_score = 0;
        for x in starting_x..ending_x {
            for y in starting_y..ending_y {
                let tree_score = self.calculate_tree_score(x, y);
                if tree_score > highest_tree_score {
                    highest_tree_score = tree_score;
                }
            }
        }

        return highest_tree_score;
    }

    fn calculate_tree_score(&self, x: i64, y: i64) -> i64 {
        self.visible_trees_from_top(x, y) *
        self.visible_trees_from_right(x, y) *
        self.visible_trees_from_bottom(x, y) *
        self.visible_trees_from_left(x, y)
    }
}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut map = Map { grid: Vec::new() };
    

    for (_, line) in reader.lines().enumerate(){
        let line = line.unwrap();
        let digits: Vec<i8> = line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect();
        map.add_row(digits);
    }

    // println!("Total visible trees: {}", map.total_visible_trees());
    println!("Highest tree score: {}", map.find_highest_tree_score());
}