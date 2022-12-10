// const INPUTS: &str = r#"
// addx 15
// addx -11
// addx 6
// addx -3
// addx 5
// addx -1
// addx -8
// addx 13
// addx 4
// noop
// addx -1
// addx 5
// addx -1
// addx 5
// addx -1
// addx 5
// addx -1
// addx 5
// addx -1
// addx -35
// addx 1
// addx 24
// addx -19
// addx 1
// addx 16
// addx -11
// noop
// noop
// addx 21
// addx -15
// noop
// noop
// addx -3
// addx 9
// addx 1
// addx -3
// addx 8
// addx 1
// addx 5
// noop
// noop
// noop
// noop
// noop
// addx -36
// noop
// addx 1
// addx 7
// noop
// noop
// noop
// addx 2
// addx 6
// noop
// noop
// noop
// noop
// noop
// addx 1
// noop
// noop
// addx 7
// addx 1
// noop
// addx -13
// addx 13
// addx 7
// noop
// addx 1
// addx -33
// noop
// noop
// noop
// addx 2
// noop
// noop
// noop
// addx 8
// noop
// addx -1
// addx 2
// addx 1
// noop
// addx 17
// addx -9
// addx 1
// addx 1
// addx -3
// addx 11
// noop
// noop
// addx 1
// noop
// addx 1
// noop
// noop
// addx -13
// addx -19
// addx 1
// addx 3
// addx 26
// addx -30
// addx 12
// addx -1
// addx 3
// addx 1
// noop
// noop
// noop
// addx -9
// addx 18
// addx 1
// addx 2
// noop
// noop
// addx 9
// noop
// noop
// noop
// addx -1
// addx 2
// addx -37
// addx 1
// addx 3
// noop
// addx 15
// addx -21
// addx 22
// addx -6
// addx 1
// noop
// addx 2
// addx 1
// noop
// addx -10
// noop
// noop
// addx 20
// addx 1
// addx 2
// addx 2
// addx -6
// addx -11
// noop
// noop
// noop
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let input = INPUTS.trim();

    let instructions = input
        .lines()
        .map(|l| l.split_once(' ').map(|(_, i)| i.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    let mut insts = instructions.iter().cycle();
    let (mut x, mut add) = (1, None);
    let (mut p1, mut p2) = (0, String::new());
    for cycle in 1..240 {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            p1 += cycle as i32 * x;
        }

        let y = (cycle - 1) % 40;
        if y == 0 {
            p2.push('\n');
        }
        p2.push(if (x - y).abs() < 2 { '█' } else { ' ' });

        match add.take() {
            Some(v) => x += v,
            None => add = insts.next().copied().unwrap(),
        }
    }

    println!("part1: {}", p1);
    println!("part2: {}", p2);
}

// fn main() {
//     let mut strengths = vec![];
//
//     let mut cycle = 1;
//     let mut x = 1;
//     for line in INPUTS.trim().lines() {
//         let (inst, val) = line.split_once(' ').unwrap_or(("noop", ""));
//         match inst {
//             "noop" => {
//                 cycle += 1;
//                 if is_target_cycles(cycle) {
//                     println!("cycle {}, x = {} => {}", cycle, x, cycle * x);
//                     let strength = cycle * x;
//                     strengths.push(strength);
//                 }
//             }
//             "addx" => {
//                 cycle += 1;
//                 if is_target_cycles(cycle) {
//                     println!("cycle {}, x = {} => {}", cycle, x, cycle * x);
//                     let strength = cycle * x;
//                     strengths.push(strength);
//                 }
//
//                 cycle += 1;
//                 x += val.parse::<i32>().unwrap();
//                 if is_target_cycles(cycle) {
//                     println!("cycle {}, x = {} => {}", cycle, x, cycle * x);
//                     let strength = cycle * x;
//                     strengths.push(strength);
//                 }
//             }
//             _ => unreachable!(),
//         }
//     }
//
//     let part1 = strengths.iter().sum::<i32>();
//     println!("part1: {}", part1);
// }
//
// fn is_target_cycles(cycle: i32) -> bool {
//     if cycle == 20 {
//         return true;
//     }
//
//     if (cycle - 20) % 40 == 0 {
//         return true;
//     }
//
//     false
// }
