use std::collections::{HashMap, VecDeque};

// const INPUTS: &str = r#"
// root: pppw + sjmn
// dbpl: 5
// cczh: sllz + lgvd
// zczc: 2
// ptdq: humn - dvpt
// dvpt: 3
// lfqf: 4
// humn: 5
// ljgn: 2
// sjmn: drzm * dbpl
// sllz: 4
// pppw: cczh / lfqf
// lgvd: ljgn * ptdq
// drzm: hmdt - zczc
// hmdt: 32
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let mut nums = HashMap::new();
    let mut lists = VecDeque::new();
    for line in INPUTS.trim().lines() {
        let (name, op_or_v) = line.split_once(": ").unwrap();
        if let Ok(v) = op_or_v.parse::<i64>() {
            nums.insert(name.to_string(), v);
        } else {
            lists.push_back(line);
        }
    }

    // println!("{:?}", nums);

    while lists.len() != 0 {
        let item = lists.pop_front().unwrap();
        println!("try to solve: {}", item);
        println!("{:?}", nums);

        let (name, op) = item.split_once(": ").unwrap();
        let lor = op.split(" ").collect::<Vec<_>>();

        if let Some(l) = nums.get(lor[0]) {
            if let Some(r) = nums.get(lor[2]) {
                let v = match lor[1] {
                    "+" => l + r,
                    "-" => l - r,
                    "*" => l * r,
                    "/" => l / r,
                    _ => unreachable!(),
                };

                if name == "root" {
                    println!("part1: {}", v);
                    break;
                } else {
                    println!("  > resolved, continue\n");
                    nums.insert(name.to_string(), v);
                    continue;
                }
            }
        }

        println!("  > no idea, continue\n");
        lists.push_back(item);
    }
}
