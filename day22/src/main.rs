// const INPUTS: &str = r#"
//         ...#
//         .#..
//         #...
//         ....
// ...#.......#
// ........#...
// ..#....#....
// ..........#.
//         ...#....
//         .....#..
//         .#......
//         ......#.
// 
// 10R5L5R10L4R5L5
// "#;
const INPUTS: &str = include_str!("../input.txt");

#[derive(Clone)]
struct Coord {
    row: i32,
    col: i32,
}

enum Direction {
    L,
    R,
    U,
    D,
}

enum Turn {
    L,
    R,
}

#[derive(PartialEq)]
enum Tile {
    Open,
    Solid,
    None,
}

enum Instruction {
    Rotate(Turn),
    Forward(u8),
}

impl Direction {
    fn score(&self) -> usize {
        match self {
            Direction::R => 0,
            Direction::D => 1,
            Direction::L => 2,
            Direction::U => 3,
        }
    }

    fn turn(self, turn: &Turn) -> Direction {
        match (self, turn) {
            (Direction::L, Turn::L) => Direction::D,
            (Direction::L, Turn::R) => Direction::U,
            (Direction::R, Turn::L) => Direction::U,
            (Direction::R, Turn::R) => Direction::D,
            (Direction::U, Turn::L) => Direction::L,
            (Direction::U, Turn::R) => Direction::R,
            (Direction::D, Turn::L) => Direction::R,
            (Direction::D, Turn::R) => Direction::L,
        }
    }

    fn offset(&self) -> Coord {
        match &self {
            Direction::L => Coord { row: 0, col: -1 },
            Direction::R => Coord { row: 0, col: 1 },
            Direction::U => Coord { row: -1, col: 0 },
            Direction::D => Coord { row: 1, col: 0 },
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Vec<Instruction>) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut instructions = Vec::new();
    let mut digits = Vec::new();
    for c in moves.chars() {
        if c.is_numeric() {
            // accumulate digits
            let digit = c.to_digit(10).unwrap() as u8;
            digits.push(digit);
        } else {
            // construct number out of digits
            // uses math to concatenate digits instead of turning them into a string first and parsing the string
            let num = digits.iter().fold(0, |num, digit| num * 10 + digit);
            digits.clear();
            instructions.push(Instruction::Forward(num));

            // parse turn
            let turn = match c {
                'L' => Turn::L,
                'R' => Turn::R,
                _ => panic!("Invalid input"),
            };
            instructions.push(Instruction::Rotate(turn));
        }
    }
    // construct number out of any remaining digits
    // uses math to concatenate digits instead of turning them into a string first and parsing the string
    let num = digits.iter().fold(0, |num, digit| num * 10 + digit);
    instructions.push(Instruction::Forward(num));

    let mut map = Vec::new();
    for line in grid.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Open,
                '#' => Tile::Solid,
                ' ' => Tile::None,
                _ => panic!("invalid input"),
            };
            row.push(tile);
        }
        map.push(row);
    }

    (map, instructions)
}

fn wrap1(map: &[Vec<Tile>], pos: &Coord, dir: &Direction) -> Coord {
    let Coord { row: dr, col: dc } = dir.offset();
    let mut curr = pos.clone();
    // while an open or solid tile exists in the map when walking in the opposite direction, update pos
    while let Some(tile) = map
        .get((curr.row - dr) as usize)
        .and_then(|row| row.get((curr.col - dc) as usize))
    {
        if *tile == Tile::None {
            break;
        }
        curr = Coord {
            row: curr.row - dr,
            col: curr.col - dc,
        };
    }

    curr
}

fn main() {
    let input = INPUTS.trim_matches('\n');
    let (map, instructions) = parse(input);
    // go to the first open position on the top row (skip the Nones)
    let start_col = map[0].iter().position(|tile| *tile == Tile::Open).unwrap() as i32;

    let mut pos = Coord {
        row: 0,
        col: start_col,
    };
    let mut dir = Direction::R;

    for inst in &instructions {
        match inst {
            Instruction::Rotate(turn) => dir = dir.turn(turn),
            Instruction::Forward(amount) => {
                // take a step "amount" times
                for _ in 0..*amount {
                    let Coord { row: dr, col: dc } = dir.offset();
                    let new_tile = map
                        .get((pos.row + dr) as usize)
                        .and_then(|row| row.get((pos.col + dc) as usize))
                        .unwrap_or(&Tile::None);

                    match new_tile {
                        // if new tile is solid, stop moving
                        Tile::Solid => break,
                        // if new tile is open, move there
                        Tile::Open => {
                            pos = Coord {
                                row: pos.row + dr,
                                col: pos.col + dc,
                            };
                        }
                        // if new tile is not found, wrap around
                        Tile::None => {
                            let new_pos = wrap1(&map, &pos, &dir);
                            // if the new_pos is solid, stop moving
                            if map[new_pos.row as usize][new_pos.col as usize] == Tile::Solid {
                                break;
                            }
                            // if the new_pos is open, move there
                            pos = new_pos;
                        }
                    }
                }
            }
        }
    }

    let part1 = 1000 * (pos.row + 1) + 4 * (pos.col + 1) + dir.score() as i32;
    println!("part1: {}", part1);
}
