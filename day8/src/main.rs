use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<u8> = Vec::new();
        for tree in line.trim().chars() {
            let tree: u8 = tree.to_digit(10).unwrap().try_into().unwrap();
            row.push(tree);
        }
        grid.push(row);
    }

    let rows = grid.len();
    let columns = grid[0].len();

    let mut visible = 0;
    for row in 0..rows {
        for col in 0..columns {
            visible += if is_visible(&grid, row, col) { 1 } else { 0 };
        }
    }

    println!("visible trees: {}", visible);
    println!("max scenic score {}", max_scenic_score(&grid));
}

fn is_visible(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let tree = grid[row][col];

    let mut visible_directions = 4;

    // left
    for i in grid[row][..col].iter() {
        if *i >= tree {
            visible_directions -= 1;
            break;
        }
    }

    // right
    for i in grid[row][col + 1..].iter() {
        if *i >= tree {
            visible_directions -= 1;
            break;
        }
    }

    // up
    for r in grid[..row].iter() {
        if r[col] >= tree {
            visible_directions -= 1;
            break;
        }
    }

    // down
    for r in grid[row + 1..].iter() {
        if r[col] >= tree {
            visible_directions -= 1;
            break;
        }
    }

    return visible_directions > 0;
}

fn max_scenic_score(grid: &Vec<Vec<u8>>) -> usize {
    let rows = grid.len();
    let columns = grid[0].len();

    let mut max = 0;
    for row in 0..rows {
        for col in 0..columns {
            let score = scenic_score(grid, row, col);
            max = if score > max { score } else { max };
        }
    }

    max
}

fn scenic_score(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let tree = grid[row][col];

    let mut score: usize;
    let mut visible: usize = 0;
    // left
    for i in grid[row][..col].iter().rev() {
        visible += 1;
        if *i >= tree {
            break;
        }
    }
    score = visible;

    // right
    visible = 0;
    for i in grid[row][col + 1..].iter() {
        visible += 1;
        if *i >= tree {
            break;
        }
    }
    score *= visible;

    // up
    visible = 0;
    for r in grid[..row].iter().rev() {
        visible += 1;
        if r[col] >= tree {
            break;
        }
    }
    score *= visible;

    // down
    visible = 0;
    for r in grid[row + 1..].iter() {
        visible += 1;
        if r[col] >= tree {
            break;
        }
    }

    score * visible
}
