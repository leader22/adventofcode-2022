const INPUTS: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;
// const INPUTS: &str = include_str!("../input.txt");

const P: usize = 10;

fn main() {
    let positions = parse_input(INPUTS.trim());
    println!("pos\n{:?}", positions);
    let (normalized_positions, min_pos, max_pos) = normalize(&positions);
    println!("normalized\n{:?}", normalized_positions);
    println!("min: {:?} / max: {:?}", min_pos, max_pos);

    let mut grid = vec![vec!['.'; max_pos.0 + P * 2]; max_pos.1 + P * 2];
    // print_grid(&grid);

    for (spos, bpos) in &normalized_positions {
        grid[spos.1 + P][spos.0 + P] = 'S';
        grid[bpos.1 + P][bpos.0 + P] = 'B';
    }
    // print_grid(&grid);

    for (spos, bpos) in &normalized_positions {
        let distance = spos.0.abs_diff(bpos.0) + spos.1.abs_diff(bpos.1);
        for x in 0..=distance {
            for y in 0..=distance - x {
                // ↘
                let dy = (spos.1 + P + y).clamp(0, max_pos.1 + P);
                let dx = (spos.0 + P + x).clamp(0, max_pos.0 + P);
                // println!("RB: {},{}", dx, dy);
                if grid[dy][dx] == '.' {
                    grid[dy][dx] = '#';
                }
                // ↙
                let dy = (spos.1 + P + y).clamp(0, max_pos.1 + P);
                let dx = (spos.0 + P - x).clamp(0, max_pos.0 + P);
                // println!("LB: {},{}", dx, dy);
                if grid[dy][dx] == '.' {
                    grid[dy][dx] = '#';
                }
                // ↗
                let dy = (spos.1 + P - y).clamp(0, max_pos.1 + P);
                let dx = (spos.0 + P + x).clamp(0, max_pos.0 + P);
                // println!("LT: {},{}", dx, dy);
                if grid[dy][dx] == '.' {
                    grid[dy][dx] = '#';
                }
                // ↖
                let dy = (spos.1 + P - y).clamp(0, max_pos.1 + P);
                let dx = (spos.0 + P - x).clamp(0, max_pos.0 + P);
                // println!("RT: {},{}", dx, dy);
                if grid[dy][dx] == '.' {
                    grid[dy][dx] = '#';
                }
            }
        }
    }
    print_grid(&grid);

    let row = 10;
    let part1 = grid[row + P].iter().filter(|&&c| c == '#').count();
    println!("part1: {}", part1);
    print_grid(&vec![grid[row + P].clone()]);
}

type Grid = Vec<Vec<char>>;
fn print_grid(grid: &Grid) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
    println!("");
}

type NPos = (usize, usize);
fn normalize(positions: &Vec<(Pos, Pos)>) -> (Vec<(NPos, NPos)>, NPos, NPos) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    for (spos, bpos) in positions {
        min_x = min_x.min(spos.0).min(bpos.0);
        min_y = min_y.min(spos.1).min(bpos.1);
    }

    // To be 0-indexed
    // min = -3 > gap = +3 / min = 4 > gap = -4
    let gap_x = min_x * -1;
    let gap_y = min_y * -1;

    let mut normalized = vec![];
    let mut max = (0, 0);
    for (spos, bpos) in positions {
        let npos = (
            ((spos.0 + gap_x) as usize, (spos.1 + gap_y) as usize),
            ((bpos.0 + gap_x) as usize, (bpos.1 + gap_y) as usize),
        );
        normalized.push(npos);

        max = (
            (max.0.max(npos.0 .0).max(npos.1 .0)) as usize,
            (max.1.max(npos.0 .1).max(npos.1 .1)) as usize,
        );
    }

    (normalized, (gap_x as usize, gap_y as usize), max)
}

type Pos = (i32, i32);
fn parse_input(input: &str) -> Vec<(Pos, Pos)> {
    let mut positions = vec![];
    for line in input.lines() {
        let (sensor, beacon) = line.split_once(": ").unwrap();
        let sensor_pos = sensor.strip_prefix("Sensor at ").unwrap();
        let beacon_pos = beacon.strip_prefix("closest beacon is at ").unwrap();

        let sensor_pos = sensor_pos
            .split_once(", ")
            .map(|pos| {
                let x = pos.0.split_once("=").unwrap().1.parse::<i32>().unwrap();
                let y = pos.1.split_once("=").unwrap().1.parse::<i32>().unwrap();
                (x, y)
            })
            .unwrap();
        let beacon_pos = beacon_pos
            .split_once(", ")
            .map(|pos| {
                let x = pos.0.split_once("=").unwrap().1.parse::<i32>().unwrap();
                let y = pos.1.split_once("=").unwrap().1.parse::<i32>().unwrap();
                (x, y)
            })
            .unwrap();
        positions.push((sensor_pos, beacon_pos));
    }

    positions
}
