use std::collections::VecDeque;

// const INPUTS: &str = r#"
// Sabqponm
// abcryxxl
// accszExk
// acctuvwj
// abdefghi
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    println!("{}", INPUTS.trim());
    let (mut grid, start, goal) = parse_input(INPUTS.trim());
    println!("{:?}", &grid);
    println!("S: {:?} > E: {:?}", start, goal);

    grid[start.0][start.1] = b'a';
    grid[goal.0][goal.1] = b'z';

    println!("part1: {}", bfs(&grid, start, goal).unwrap());
}

type Grid = Vec<Vec<u8>>;
type Pos = (usize, usize);
fn parse_input(input: &str) -> (Grid, Pos, Pos) {
    let mut grid = vec![];
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for (y, line) in input.lines().enumerate() {
        grid.push(vec![]);
        for (x, &p) in line.as_bytes().iter().enumerate() {
            if p == "S".as_bytes()[0] {
                start = (y, x);
            }
            if p == "E".as_bytes()[0] {
                goal = (y, x);
            }
            grid.last_mut().unwrap().push(p);
        }
    }
    (grid, start, goal)
}

fn bfs(grid: &[Vec<u8>], start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    let max_y = grid.len();
    let max_x = grid[0].len();
    let mut visited = vec![vec![false; max_x]; max_y];
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    while let Some(((x, y), len)) = queue.pop_front() {
        if (x, y) == goal {
            return Some(len);
        }

        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            let Some(&next) = grid.get(nx).and_then(|row| row.get(ny)) else { continue };
            if grid[x][y] + 1 >= next && !visited[nx][ny] {
                visited[nx][ny] = true;
                queue.push_back(((nx, ny), len + 1));
            }
        }
    }

    None
}
