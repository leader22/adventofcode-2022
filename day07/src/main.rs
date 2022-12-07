// const INPUTS: &str = r#"
// $ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k
// "#;
const INPUTS: &str = include_str!("../input.txt");

use std::{collections::HashMap, path::PathBuf};

fn main() {
    let mut directory_sizes = HashMap::new();
    let mut dir_queue = vec![];
    let mut pwd = PathBuf::new();

    for line in INPUTS.trim().lines() {
        if line.starts_with("$ cd") && line.contains("..") == false {
            let dir = line.split_once("$ cd ").unwrap().1;
            pwd.push(dir);
            dir_queue.push(pwd.to_str().unwrap().to_string());
        } else if line.starts_with("$ cd ..") {
            dir_queue.pop();
            pwd.pop();
        } else {
            let size = line.split_once(' ').unwrap().0.parse::<i64>().unwrap_or(0);
            for dir in dir_queue.clone() {
                directory_sizes
                    .entry(dir)
                    .and_modify(|e| *e += size)
                    .or_insert(0);
            }
        }
    }
    println!("{:?}", directory_sizes);

    let part1 = directory_sizes
        .values()
        .filter(|&&size| size <= 100_000)
        .sum::<i64>();
    println!("part1: {}", part1);

    let unused = 30_000_000 - (70_000_000 - directory_sizes.get("/").unwrap());
    let part2 = directory_sizes
        .values()
        .filter(|&&size| size >= unused)
        .min()
        .unwrap();
    println!("part2: {}", part2);
}
