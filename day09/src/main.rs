use std::collections::HashSet;

// const INPUTS: &str = r#"
// R 4
// U 4
// L 3
// D 1
// R 4
// D 1
// L 5
// R 2
// "#;
const INPUTS: &str = include_str!("../input.txt");

type Pos = (i32, i32);
fn main() {
    let moves = parse_input(INPUTS.trim());
    println!("{:?}", &moves);

    let part1 = simulate(&moves, 2);
    println!("part1: {}", part1);

    let part2 = simulate(&moves, 10);
    println!("part2: {}", part2);
}

type Moves = Vec<(char, i32)>;
fn parse_input(input: &str) -> Moves {
    input
        .lines()
        .map(|line| {
            let (dir, _steps) = line.split_once(' ').unwrap();
            let steps = _steps.parse::<i32>().unwrap();
            (dir.chars().last().unwrap(), steps)
        })
        .collect::<Vec<_>>()
}

fn simulate(moves: &Moves, knots_len: usize) -> usize {
    let mut knots: Vec<Pos> = vec![];
    for _ in 0..knots_len {
        knots.push((0, 0));
    }

    let mut visited: HashSet<String> = HashSet::new();
    let last = *knots.last().unwrap();
    visited.insert(format!("{:?}", last));

    for (dir, step) in moves {
        for _ in 0..*step {
            match dir {
                'L' => knots[0].0 -= 1,
                'R' => knots[0].0 += 1,
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                _ => unreachable!(),
            }

            for t in 1..knots_len {
                // Update tail(latter) pos if needed
                if (knots[t - 1].0 - knots[t].0).abs() >= 2
                    || (knots[t - 1].1 - knots[t].1).abs() >= 2
                {
                    knots[t].0 += (knots[t - 1].0 - knots[t].0).signum();
                    knots[t].1 += (knots[t - 1].1 - knots[t].1).signum();
                }
            }

            let last = *knots.last().unwrap();
            visited.insert(format!("{:?}", last));
        }
    }

    visited.len()
}
