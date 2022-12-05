// const INPUTS: &str = r#"
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3
//
// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let (stacks_of_crates, rearragenement_procedures) =
        INPUTS.trim_matches('\n').split_once("\n\n").unwrap();

    let mut stacks_of_crates_lines = stacks_of_crates.lines().rev();
    let num_of_stacks = stacks_of_crates_lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    // println!("{} stacks", num_of_stacks);

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..=num_of_stacks {
        stacks.push(vec![]);
    }
    for row in stacks_of_crates_lines {
        let mut stacks_pos = 1_usize;
        for col in row
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|cols| cols[1])
        {
            if col != ' ' {
                stacks[stacks_pos].push(col);
            }
            stacks_pos += 1;
        }
    }

    // println!("{:#?}", stacks);

    let procedures = rearragenement_procedures.lines().map(|line| {
        let splits = line.split_whitespace().collect::<Vec<_>>();
        let num = splits[1].parse::<i32>().unwrap();
        let from = splits[3].parse::<usize>().unwrap();
        let to = splits[5].parse::<usize>().unwrap();
        (num, from, to)
    });
    // println!("{:#?}", procedures.clone().collect::<Vec<_>>());

    let mut stacks_part1 = stacks.clone();
    for (num, from, to) in procedures.clone() {
        let mut from_stack = stacks_part1.get(from).unwrap().clone();
        let mut to_stack = stacks_part1.get(to).unwrap().clone();

        for _ in 1..=num {
            let c = from_stack.pop().unwrap();
            to_stack.push(c);
        }

        stacks_part1[from] = from_stack;
        stacks_part1[to] = to_stack;
    }

    // println!("{:#?}", stacks_part1);

    let part1 = stacks_part1
        .iter_mut()
        .filter_map(|stack| stack.pop())
        .collect::<String>();
    println!("{:?}", part1);

    let mut stacks_part2 = stacks.clone();
    for (num, from, to) in procedures.clone() {
        let mut from_stack = stacks_part2.get(from).unwrap().clone();
        let mut to_stack = stacks_part2.get(to).unwrap().clone();

        let mut temp = vec![];
        for _ in 1..=num {
            let c = from_stack.pop().unwrap();
            temp.push(c);
        }
        temp.reverse();
        for c in temp {
            to_stack.push(c);
        }

        stacks_part2[from] = from_stack;
        stacks_part2[to] = to_stack;
    }

    // println!("{:#?}", stacks_part2);

    let part2 = stacks_part2
        .iter_mut()
        .filter_map(|stack| stack.pop())
        .collect::<String>();
    println!("{:?}", part2);
}
