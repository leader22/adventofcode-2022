use std::collections::HashSet;

// const INPUTS: &str = r#"
// 2,2,2
// 1,2,2
// 3,2,2
// 2,1,2
// 2,3,2
// 2,2,1
// 2,2,3
// 2,2,4
// 2,2,6
// 1,2,5
// 3,2,5
// 2,1,5
// 2,3,5
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let cubes = parse_input(INPUTS.trim());
    let len = cubes.len();
    println!("{:?}", cubes);

    let mut covered = 0;
    let seen: HashSet<(i32, i32, i32)> = HashSet::from_iter(cubes.clone().into_iter());

    for cube in cubes {
        for xyz in 0..3 {
            for delta in [-1, 1] {
                let mut s = [0; 3];
                s[xyz] = delta;

                if seen.contains(&(cube.0 + s[0], cube.1 + s[1], cube.2 + s[2])) {
                    covered += 1;
                }
            }
        }
    }

    let part1 = len * 6 - covered;
    println!("part1: {}", part1);
}

fn parse_input(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|xyz| {
            let mut iter = xyz.split(',');
            (
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}
