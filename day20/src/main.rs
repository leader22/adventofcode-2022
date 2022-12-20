// const INPUTS: &str = r#"
// 1
// 2
// -3
// 3
// -2
// 0
// 4
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let numbers = INPUTS
        .trim()
        .lines()
        .flat_map(|line| line.parse())
        .collect::<Vec<_>>();
    println!("{:?}", numbers);

    let part1 = mix(&numbers);
    println!("part1: {}", part1);
}

fn mix(nums: &[i64]) -> i64 {
    let mut ans = (0..nums.len()).collect::<Vec<_>>();
    for (idx, &n) in nums.iter().enumerate() {
        let pos = ans.iter().position(|&y| y == idx).unwrap();
        ans.remove(pos);

        let new_idx = (pos as i64 + n).rem_euclid(ans.len() as i64) as usize;
        ans.insert(new_idx, idx);
    }

    let orig_zero_idx = nums.iter().position(|&n| n == 0).unwrap();
    let zero_idx = ans.iter().position(|&idx| idx == orig_zero_idx).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| nums[ans[(zero_idx + i) % ans.len()]])
        .sum()
}
