// const INPUTS: &str = r#"
// 498,4 -> 498,6 -> 496,6
// 503,4 -> 502,4 -> 502,9 -> 494,9
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let (positions, min, max) = parse_input(INPUTS.trim());
    // println!("{:?}", &positions);
    // println!("min/max {:?}", (&min, &max));

    let mut grid = init_grid(min, max, positions);
    print_grid(&grid);
    println!("start -----\n");

    for ans in 1.. {
        let (mut x, mut y) = (500 - min.0, 0);
        while y <= max.1 {
            if grid[y + 1][x] == '.' {
                y += 1;
            } else if grid[y + 1][x - 1] == '.' {
                x -= 1;
                y += 1;
            } else if grid[y + 1][x + 1] == '.' {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        grid[y][x] = 'o';
        print_grid(&grid);
        println!("sand: {}\n", ans);
    }
}

type Grid = Vec<Vec<char>>;
fn init_grid(min: Pos, max: Pos, positions: Vec<Pos>) -> Grid {
    let mut grid = vec![vec!['.'; max.0 - min.0 + 1]; max.1 + 1];

    for (x, y) in positions {
        let (y_idx, x_idx) = (y, x - min.0);
        grid[y_idx][x_idx] = '#';
    }

    grid
}

type Pos = (usize, usize);
fn parse_input(input: &str) -> (Vec<Pos>, Pos, Pos) {
    let mut min = (999, 999);
    let mut max = (0, 0);
    let mut positions = vec![];
    for line in input.lines() {
        let points = line.split(" -> ");

        let points = points
            .map(|point| {
                let point = point.split_once(',').unwrap();
                (
                    point.0.parse::<usize>().unwrap(),
                    point.1.parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        for pair in points.windows(2) {
            let (px, py) = pair[0];
            let (nx, ny) = pair[1];

            if px == nx {
                for y in py.min(ny)..=py.max(ny) {
                    positions.push((px, y));
                    min = (min.0.min(px), min.1.min(y));
                    max = (max.0.max(px), max.1.max(y));
                }
            }
            if py == ny {
                for x in px.min(nx)..=px.max(nx) {
                    positions.push((x, py));
                    min = (min.0.min(x), min.1.min(py));
                    max = (max.0.max(x), max.1.max(py));
                }
            }
        }
    }

    (positions, min, max)
}

fn print_grid(grid: &Grid) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
}
