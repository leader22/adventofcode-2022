use std::collections::{HashMap, HashSet};

// const INPUTS: &str = r#"
// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
// Valve BB has flow rate=13; tunnels lead to valves CC, AA
// Valve CC has flow rate=2; tunnels lead to valves DD, BB
// Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
// Valve EE has flow rate=3; tunnels lead to valves FF, DD
// Valve FF has flow rate=0; tunnels lead to valves EE, GG
// Valve GG has flow rate=0; tunnels lead to valves FF, HH
// Valve HH has flow rate=22; tunnel leads to valve GG
// Valve II has flow rate=0; tunnels lead to valves AA, JJ
// Valve JJ has flow rate=21; tunnel leads to valve II
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let valve_map = parse_input(INPUTS.trim());
    println!("{:?}", valve_map);

    let part1 = dfs(
        1,
        "AA",
        0,
        0,
        &HashSet::new(),
        &valve_map,
        &mut HashMap::new(),
    )
    .unwrap();
    println!("part1: {}", part1);
}

fn dfs(
    minute: u32,
    current_location: &str,
    flow_rate: u64,
    current_score: u64,
    open_valves: &HashSet<String>,
    valve_map: &HashMap<String, Valve>,
    cache: &mut HashMap<(u32, String, u64), u64>,
) -> Option<u64> {
    if minute > 30 {
        return Some(current_score);
    }

    let cache_key = (minute, current_location.to_string(), flow_rate);
    if let Some(cached_value) = cache.get(&cache_key) {
        if *cached_value >= current_score {
            return None;
        }
    }
    cache.insert(cache_key, current_score);

    let current_valve = valve_map.get(current_location).unwrap();

    let best_result_open_current = if current_valve.0 > 0 && !open_valves.contains(current_location)
    {
        let mut new_open_valves = open_valves.iter().cloned().collect::<HashSet<_>>();
        new_open_valves.insert(current_location.to_string());

        let new_score = current_score + flow_rate;
        let new_flow_rate = flow_rate + current_valve.0;
        dfs(
            minute + 1,
            current_location,
            new_flow_rate,
            new_score,
            &new_open_valves,
            valve_map,
            cache,
        )
    } else {
        None
    };

    let best_result_down_tunnels = current_valve
        .1
        .iter()
        .filter_map(|next_valve_name| {
            dfs(
                minute + 1,
                next_valve_name,
                flow_rate,
                current_score + flow_rate,
                open_valves,
                valve_map,
                cache,
            )
        })
        .max();

    best_result_down_tunnels.max(best_result_open_current)
}

type Valve = (u64, Vec<String>);
fn parse_input(input: &str) -> HashMap<String, Valve> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("; ").unwrap();
        let (a, flow_rate) = a.split_once(" has flow rate=").unwrap();
        let (_, valve_name) = a.split_once(' ').unwrap();

        let (_, b) = b.split_once(" to ").unwrap();
        let (_, tunnels) = b.split_once(' ').unwrap();

        let valve = (
            flow_rate.parse().unwrap(),
            tunnels.split(", ").map(|s| s.to_string()).collect(),
        );

        map.insert(valve_name.to_string(), valve);
    }

    map
}
