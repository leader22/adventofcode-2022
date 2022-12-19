use std::collections::HashSet;

// const INPUTS: &str = r#"
// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
// Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let blueprints = parse_input(INPUTS.trim());
    println!("{:?}", blueprints);

    let mut part1 = 0;
    for bp in &blueprints {
        let geodes = simulate(bp) as u32;
        println!("ID {} can open {} geodes\n", bp.id, geodes);

        part1 += geodes * bp.id;
    }

    println!("part1: {}", part1);
}

const TIME: i32 = 24;
const THRESHOLD: usize = 100;

type Choice = ([usize; 4], [usize; 4], usize);

fn simulate(bp: &Blueprint) -> usize {
    // Find best options.
    let resources: [usize; 4] = [0; 4];
    let mut robots: [usize; 4] = [0; 4];
    robots[0] = 1; // Start 1 ore

    let mut choices: HashSet<Choice> = HashSet::new();
    choices.insert((resources, robots, 0));

    let mut best = 0;
    let mut top = THRESHOLD;
    for m in 1..TIME + 1 {
        println!("Choices: {}@{} => {}", bp.id, m, choices.len());

        let mut new_choices: HashSet<Choice> = HashSet::new();
        for choice in &choices {
            // Skip choices that are performing badly.
            if choice.2 > top {
                top = choice.2;
            }
            if choice.2 < top - THRESHOLD {
                continue;
            }

            // Valid choices from here.
            for (item, cost) in bp.costs.iter().enumerate() {
                let mut affordable = true;
                for (i, material) in cost.iter().enumerate() {
                    if choice.0[i] < *material {
                        affordable = false;
                    }
                }
                if affordable {
                    // Spend resources
                    let mut new_resources = choice.0.clone();
                    for (i, material) in cost.iter().enumerate() {
                        new_resources[i] -= material;
                    }
                    // Get new robot
                    let mut new_robots = choice.1.clone();
                    new_robots[item] += 1;
                    // Add resources
                    let mut new_top = choice.2;
                    for (i, r) in new_resources.iter_mut().enumerate() {
                        *r += choice.1[i]; // Past robots
                        new_top += choice.1[i] * (i + 1); // Add resources
                    }
                    new_choices.insert((new_resources, new_robots, new_top));
                }
            }
            // Add do nothing choice
            let mut new_resources = choice.0.clone();
            // Add resources
            let mut new_top = choice.2;
            for (i, r) in new_resources.iter_mut().enumerate() {
                *r += choice.1[i]; // Past robots
                new_top += choice.1[i] * (i + 1); // Add resources
            }
            new_choices.insert((new_resources, choice.1.clone(), new_top));
        }
        choices = new_choices;
    }

    // Check best choice
    for choice in choices {
        best = best.max(choice.0[3]);
    }

    best
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    costs: [[usize; 3]; 4],
}
fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let (id, robots) = line.split_once(": ").unwrap();

            let id = id
                .strip_prefix("Blueprint ")
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let mut defs = robots.split_whitespace().flat_map(|v| v.parse::<usize>());
            let ore_robot_cost = [defs.next().unwrap(), 0, 0];
            let clay_robot_cost = [defs.next().unwrap(), 0, 0];
            let obs_robot_cost = [defs.next().unwrap(), defs.next().unwrap(), 0];
            let geode_robot_cost = [defs.next().unwrap(), 0, defs.next().unwrap()];

            Blueprint {
                id,
                costs: [
                    ore_robot_cost,
                    clay_robot_cost,
                    obs_robot_cost,
                    geode_robot_cost,
                ],
            }
        })
        .collect::<Vec<_>>()
}
