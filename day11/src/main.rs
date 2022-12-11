use std::collections::HashMap;

// const INPUTS: &str = r#"
// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
//
// Monkey 1:
//   Starting items: 54, 65, 75, 74
//   Operation: new = old + 6
//   Test: divisible by 19
//     If true: throw to monkey 2
//     If false: throw to monkey 0
//
// Monkey 2:
//   Starting items: 79, 60, 97
//   Operation: new = old * old
//   Test: divisible by 13
//     If true: throw to monkey 1
//     If false: throw to monkey 3
//
// Monkey 3:
//   Starting items: 74
//   Operation: new = old + 3
//   Test: divisible by 17
//     If true: throw to monkey 0
//     If false: throw to monkey 1
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let (ids, mut map) = parse_input(INPUTS.trim());
    // println!("{:?}", ids);
    // println!("{:#?}", map);

    let mut inspected = HashMap::new();
    let mut passed = HashMap::new();
    for round in 1..=20 {
        println!("<Round-{}>", round);

        for id in &ids {
            println!("<TurnFor-{}>", id);
            let monkey = map.get_mut(id).unwrap();

            if let Some(levels) = passed.get(id) {
                monkey.0.extend(levels);
                passed.remove(id);
            }

            println!("  levels: {:?}", monkey.0);
            inspected
                .entry(id)
                .and_modify(|c| *c += monkey.0.len())
                .or_insert(monkey.0.len());
            for lv in &monkey.0 {
                let next_lv = update_lv(*lv, &monkey.1);
                let next_id = throw_to(next_lv, &monkey.2);

                // map.get_mut(&next_id).unwrap().0.push(next_lv);
                passed
                    .entry(next_id)
                    .and_modify(|vec: &mut Vec<i32>| {
                        vec.push(next_lv);
                    })
                    .or_insert(vec![next_lv]);
            }
            monkey.0 = vec![];

            println!("  passed: {:?}", passed);
            println!("</TurnFor-{}>", id);
        }
        println!("</Round-{}>", round);
    }

    println!("Inspect: {:?}", inspected);

    let mut times = inspected.values().collect::<Vec<_>>();
    times.sort_unstable();
    let part1 = times.into_iter().rev().take(2).product::<usize>();
    println!("part1: {:?}", part1);
}

type Monkey = (Vec<i32>, Vec<String>, (Vec<String>, (i32, i32)));
fn parse_input(input: &str) -> (Vec<i32>, HashMap<i32, Monkey>) {
    let mut monkey_ids = vec![];
    let mut monkey_map = HashMap::new();

    let monkey_defs = input.split("\n\n");
    for monkey in monkey_defs {
        let mut lines = monkey.lines();
        let id = lines
            .next()
            .unwrap()
            .strip_prefix("Monkey ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        println!("ID: {}", id);

        let levels = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        println!("LVs: {:?}", levels);

        let operation = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = ")
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        println!("OP: {:?}", operation);

        let test = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: ")
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let true_id = lines
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let false_id = lines
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        println!("TEST: {:?}", test);
        println!("NEXT: {} or {}", true_id, false_id);

        println!("");

        monkey_ids.push(id);
        monkey_map.insert(id, (levels, operation, (test, (true_id, false_id))));
    }

    (monkey_ids, monkey_map)
}

fn update_lv(lv: i32, op: &Vec<String>) -> i32 {
    let left = match op[0].as_ref() {
        "old" => lv,
        num => num.parse::<i32>().unwrap(),
    };
    let right = match op[2].as_ref() {
        "old" => lv,
        num => num.parse::<i32>().unwrap(),
    };

    let lv = match op[1].as_ref() {
        "+" => left + right,
        "*" => left * right,
        _ => unreachable!(),
    };

    lv / 3
}

fn throw_to(lv: i32, (test, (true_id, false_id)): &(Vec<String>, (i32, i32))) -> i32 {
    if test[0] != "divisible" {
        panic!();
    }

    let v = test[2].parse::<i32>().unwrap();

    if lv % v == 0 {
        *true_id
    } else {
        *false_id
    }
}
