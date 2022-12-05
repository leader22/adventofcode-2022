// const INPUTS: &str = r#"
// 2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let pairs = INPUTS.trim().lines().map(|line| {
        let (a, b) = line.split_once(',').unwrap();
        let (a_section_start, a_section_end) = a.split_once('-').unwrap();
        let (b_section_start, b_section_end) = b.split_once('-').unwrap();

        let a_range = (
            a_section_start.parse::<i32>().unwrap(),
            a_section_end.parse::<i32>().unwrap(),
        );
        let b_range = (
            b_section_start.parse::<i32>().unwrap(),
            b_section_end.parse::<i32>().unwrap(),
        );

        (a_range, b_range)
    });
    // println!("{:?}", pairs.clone().collect::<Vec<_>>());

    let part1 = pairs.filter(|(a, b)| is_within(*a, *b)).count();
    println!("{}", part1);
}

fn is_within(a: (i32, i32), b: (i32, i32)) -> bool {
    // a in b
    if b.0 <= a.0 && a.1 <= b.1 {
        return true;
    }
    // b in a
    if a.0 <= b.0 && b.1 <= a.1 {
        return true;
    }

    false
}
