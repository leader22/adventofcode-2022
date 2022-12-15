// const INPUTS: &str = r#"
// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
// Sensor at x=9, y=16: closest beacon is at x=10, y=16
// Sensor at x=13, y=2: closest beacon is at x=15, y=3
// Sensor at x=12, y=14: closest beacon is at x=10, y=16
// Sensor at x=10, y=20: closest beacon is at x=10, y=16
// Sensor at x=14, y=17: closest beacon is at x=10, y=16
// Sensor at x=8, y=7: closest beacon is at x=2, y=10
// Sensor at x=2, y=0: closest beacon is at x=2, y=10
// Sensor at x=0, y=11: closest beacon is at x=2, y=10
// Sensor at x=20, y=14: closest beacon is at x=25, y=17
// Sensor at x=17, y=20: closest beacon is at x=21, y=22
// Sensor at x=16, y=7: closest beacon is at x=15, y=3
// Sensor at x=14, y=3: closest beacon is at x=15, y=3
// Sensor at x=20, y=1: closest beacon is at x=15, y=3
// "#;
const INPUTS: &str = include_str!("../input.txt");

const ROW_IDX: i32 = 2000000;

fn main() {
    let positions = parse_input(INPUTS.trim());
    println!("pos\n{:?}", positions);

    let mut compressed = positions
        .iter()
        .map(|(sp, _, d)| (sp.0, d - (ROW_IDX - sp.1).abs()))
        .filter(|(_, edge)| *edge >= 0)
        .flat_map(|(x, edge)| [(x - edge, true), (x + edge + 1, false)])
        .collect::<Vec<_>>();
    compressed.sort_unstable();

    let (mut ans, mut prev, mut inside) = (-1, 0, 0);
    for &(x, start) in &compressed {
        if inside > 0 {
            ans += x - prev
        }
        inside += if start { 1 } else { -1 };
        prev = x;
    }

    let part1 = ans;
    println!("part1: {}", part1);
}

type Pos = (i32, i32);
fn parse_input(input: &str) -> Vec<(Pos, Pos, i32)> {
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

        let distance = (sensor_pos.0 - beacon_pos.0).abs() + (sensor_pos.1 - beacon_pos.1).abs();
        positions.push((sensor_pos, beacon_pos, distance));
    }

    positions
}
