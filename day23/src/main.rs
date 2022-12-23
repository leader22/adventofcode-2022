use std::collections::{HashMap, HashSet};

// const INPUTS: &str = r#"
// ..............
// ..............
// .......#......
// .....###.#....
// ...#...#.#....
// ....#...##....
// ...#.###......
// ...##.#.##....
// ....#..#......
// ..............
// ..............
// ..............
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let mut state = HashSet::new();
    for (r, row) in INPUTS.trim().lines().enumerate() {
        for (c, v) in row.chars().enumerate() {
            if v == '#' {
                state.insert((r as i32, c as i32));
            }
        }
    }

    for round in 0.. {
        let mut proposals = HashMap::<_, Vec<_>>::new();
        for &(x, y) in &state {
            // Check adjucent to take a move
            let ns = [
                state.contains(&(x - 1, y - 1)), // NW
                state.contains(&(x - 1, y)),     // N
                state.contains(&(x - 1, y + 1)), // NE
                state.contains(&(x, y - 1)),     // W
                state.contains(&(x, y + 1)),     // E
                state.contains(&(x + 1, y - 1)), // SW
                state.contains(&(x + 1, y)),     // S
                state.contains(&(x + 1, y + 1)), // SE
            ];
            if ns.iter().all(|b| !b) {
                continue;
            }

            let props = [
                // N
                (!ns[0] && !ns[1] && !ns[2], (x - 1, y)),
                // S
                (!ns[5] && !ns[6] && !ns[7], (x + 1, y)),
                // W
                (!ns[0] && !ns[3] && !ns[5], (x, y - 1)),
                // E
                (!ns[2] && !ns[4] && !ns[7], (x, y + 1)),
            ];
            for i in 0..4 {
                let (free, pos) = props[(round + i) % 4];
                if free {
                    proposals.entry(pos).or_default().push((x, y));
                    break;
                }
            }
        }

        for (pos, props) in proposals {
            if props.len() != 1 {
                continue;
            }

            state.remove(&props[0]);
            state.insert(pos);
        }

        if round == 9 {
            break;
        }
    }

    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    for (x, y) in &state {
        min = (min.0.min(*x), min.1.min(*y));
        max = (max.0.max(*x), max.1.max(*y));
    }

    let mut part1 = 0;
    for x in min.0..=max.0 {
        for y in min.1..=max.1 {
            if state.contains(&(x, y)) {
                continue;
            }
            part1 += 1;
        }
    }
    println!("part1: {}", part1);
}
