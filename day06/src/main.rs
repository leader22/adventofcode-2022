// const INPUTS: &str = r#"
// nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let chars = INPUTS.trim().chars().collect::<Vec<_>>();

    for (idx, chars) in chars.windows(4).enumerate() {
        // println!("{:?}", chars);
        if is_all_different(chars) {
            println!("part1: {}", idx + 4);
            break;
        }
    }

    for (idx, chars) in chars.windows(14).enumerate() {
        // println!("{:?}", chars);
        if is_all_different(chars) {
            println!("part2: {}", idx + 14);
            break;
        }
    }
}

fn is_all_different(chars: &[char]) -> bool {
    let mut found = [0; 26];

    for c in chars {
        let idx = (*c as u8 - 97) as usize;
        if found[idx] != 0 {
            return false;
        }
        found[idx] = 1;
    }

    true
}
