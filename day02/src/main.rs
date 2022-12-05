// const INPUTS: &str = r#"
// A Y
// B X
// C Z
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let (lost, draw, win) = (0, 3, 6);
    let (rock, paper, scissors) = (1, 2, 3);

    let guide_hands = INPUTS
        .trim()
        .lines()
        .map(|line| line.split_once(' ').unwrap());

    let part1 = guide_hands
        .clone()
        .map(|hand| match hand {
            ("A", "X") => rock + draw,
            ("A", "Y") => paper + win,
            ("A", "Z") => scissors + lost,
            ("B", "X") => rock + lost,
            ("B", "Y") => paper + draw,
            ("B", "Z") => scissors + win,
            ("C", "X") => rock + win,
            ("C", "Y") => paper + lost,
            ("C", "Z") => scissors + draw,
            _ => unreachable!(),
        })
        // .map(|x| dbg!(x))
        .sum::<i32>();
    println!("part1: {}", part1);

    let part2 = guide_hands
        .map(|hands| match hands {
            ("A", "X") => lost + scissors,
            ("A", "Y") => draw + rock,
            ("A", "Z") => win + paper,
            ("B", "X") => lost + rock,
            ("B", "Y") => draw + paper,
            ("B", "Z") => win + scissors,
            ("C", "X") => lost + paper,
            ("C", "Y") => draw + scissors,
            ("C", "Z") => win + rock,
            _ => unreachable!(),
        })
        .sum::<i32>();
    println!("part2: {}", part2);
}
