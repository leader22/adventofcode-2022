// const INPUTS: &str = r#"
// 30373
// 25512
// 65332
// 33549
// 35390
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    println!("{}", INPUTS.trim());
    let grid = to_tree_grid(INPUTS.trim());

    let mut part1 = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            let is_visible = is_visible(x, y, &grid);
            println!("[{}][{}] = {} ? {}", y, x, h, is_visible);
            if is_visible {
                part1 += 1;
            }
        }
    }

    println!("part1: {}", part1);

    let mut part2 = 0_u32;
    for (y, row) in grid.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            let score = scenic_score(x, y, &grid);
            println!("[{}][{}] = {} ? {}", y, x, h, score);
            part2 = part2.max(score);
        }
    }

    println!("part2: {}", part2);
}

type Grid = Vec<Vec<u32>>;
fn to_tree_grid(input: &str) -> Grid {
    let mut grid = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for h in line.chars() {
            let height = h.to_digit(10).unwrap();
            row.push(height);
        }
        grid.push(row);
    }

    grid
}

fn is_edge(x: usize, y: usize, grid: &Grid) -> bool {
    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;

    if y == 0 || y == max_y || x == 0 || x == max_x {
        return true;
    }
    false
}

fn is_visible(x: usize, y: usize, grid: &Grid) -> bool {
    if is_edge(x, y, grid) {
        return true;
    }

    let h = grid[y][x];

    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;
    // ←
    if grid[y][0..x].iter().all(|&v| v < h) {
        return true;
    }
    // →
    if grid[y][(x + 1)..=max_x].iter().all(|&v| v < h) {
        return true;
    }
    // ↑
    if grid[0..y].iter().map(|row| row[x]).all(|v| v < h) {
        return true;
    }
    // ↓
    if grid[(y + 1)..=max_y]
        .iter()
        .map(|row| row[x])
        .all(|v| v < h)
    {
        return true;
    }

    false
}

fn scenic_score(x: usize, y: usize, grid: &Grid) -> u32 {
    let h = grid[y][x];

    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;
    let mut scores = [0; 4];
    // ←
    for &v in grid[y][0..x].iter().rev() {
        scores[0] += 1;
        if v >= h {
            break;
        }
    }
    // →
    for &v in grid[y][(x + 1)..=max_x].iter() {
        scores[1] += 1;
        if v >= h {
            break;
        }
    }
    // ↑
    for v in grid[0..y].iter().map(|row| row[x]).rev() {
        scores[2] += 1;
        if v >= h {
            break;
        }
    }
    // ↓
    for v in grid[(y + 1)..=max_y].iter().map(|row| row[x]) {
        scores[3] += 1;
        if v >= h {
            break;
        }
    }

    scores.iter().product()
}
