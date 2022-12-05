// const INPUTS: &str = r#"
// 1000
// 2000
// 3000
//
// 4000
//
// 5000
// 6000
//
// 7000
// 8000
// 9000
//
// 10000
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let elves_with_calories = INPUTS
        .trim()
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|calories| calories.iter().sum::<i32>());
    println!("{:?}", elves_with_calories.clone().collect::<Vec<i32>>());

    let mut sorted_calories = elves_with_calories.collect::<Vec<i32>>();
    sorted_calories.sort_unstable();

    let no1 = sorted_calories.pop().unwrap();
    let no2 = sorted_calories.pop().unwrap();
    let no3 = sorted_calories.pop().unwrap();

    println!("{}, {}, {}", no1, no2, no3);
    println!("sum: {}", no1 + no2 + no3);
}
