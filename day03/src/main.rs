// const INPUTS: &str = r#"
// vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg
// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let items_in_rucksacks = INPUTS
        .trim()
        .lines()
        .map(|line| line.split_at(line.len() / 2));
    // println!("{:#?}", items_in_rucksacks.clone().collect::<Vec<_>>());

    let common_items = items_in_rucksacks.filter_map(|(a, b)| find_common_chars(a, b));
    // println!("{:#?}", common_items.clone().collect::<Vec<_>>());

    let part1 = common_items.map(char_to_priority).sum::<usize>();
    println!("{}", part1);

    let part2 = INPUTS
        .trim()
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .filter_map(|group| find_common_chars2(group[0], group[1], group[2]))
        .map(char_to_priority)
        .sum::<usize>();
    println!("{}", part2);
}

// c as usize
// A =  65
// Z =  90
// a =  97
// z = 122
fn find_common_chars(s1: &str, s2: &str) -> Option<char> {
    let mut lower = [0; 26];
    let mut upper = [0; 26];
    for c in s1.chars() {
        if c.is_uppercase() {
            lower[c as usize - 65] += 1;
        } else {
            upper[c as usize - 97] += 1;
        }
    }
    for c in s2.chars() {
        if c.is_uppercase() {
            if lower[c as usize - 65] != 0 {
                return Some(c);
            }
        } else {
            if upper[c as usize - 97] != 0 {
                return Some(c);
            }
        }
    }
    None
}

fn find_common_chars2(s1: &str, s2: &str, s3: &str) -> Option<char> {
    let mut lower1 = [0; 26];
    let mut upper1 = [0; 26];
    for c in s1.chars() {
        if c.is_uppercase() {
            lower1[c as usize - 65] = 1;
        } else {
            upper1[c as usize - 97] = 1;
        }
    }

    let mut lower2 = [0; 26];
    let mut upper2 = [0; 26];
    for c in s2.chars() {
        if c.is_uppercase() {
            lower2[c as usize - 65] = 1;
        } else {
            upper2[c as usize - 97] = 1;
        }
    }

    let mut lower3 = [0; 26];
    let mut upper3 = [0; 26];
    for c in s3.chars() {
        if c.is_uppercase() {
            lower3[c as usize - 65] = 1;
        } else {
            upper3[c as usize - 97] = 1;
        }
    }

    for i in 0..=25 {
        if lower1[i] + lower2[i] + lower3[i] == 3 {
            return Some((i as u8 + 65) as char);
        }
        if upper1[i] + upper2[i] + upper3[i] == 3 {
            return Some((i as u8 + 97) as char);
        }
    }

    None
}

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn char_to_priority(c: char) -> usize {
    if c.is_uppercase() {
        c as usize - 65 + 27
    } else {
        c as usize - 97 + 1
    }
}
