use std::collections::HashMap;

// const INPUTS: &str = r#"
// >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
// "#;
const INPUTS: &str = include_str!("../input.txt");

type Coord = (isize, isize);

#[derive(Copy, Clone, Debug)]
enum RockShape {
    Flat,
    Plus,
    BackwardL,
    I,
    O,
}

impl RockShape {
    fn all() -> Vec<RockShape> {
        [
            RockShape::Flat,
            RockShape::Plus,
            RockShape::BackwardL,
            RockShape::I,
            RockShape::O,
        ]
        .into()
    }

    fn cells(self) -> impl Iterator<Item = Coord> {
        match self {
            RockShape::Flat => [(0, 0), (1, 0), (2, 0), (3, 0)].iter().copied(),
            RockShape::Plus => [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)].iter().copied(),
            RockShape::BackwardL => [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].iter().copied(),
            RockShape::I => [(0, 0), (0, 1), (0, 2), (0, 3)].iter().copied(),
            RockShape::O => [(0, 0), (1, 0), (0, 1), (1, 1)].iter().copied(),
        }
    }
}

#[derive(Default, Clone)]
struct Well {
    columns: [HashMap<isize, bool>; 7],
}

impl Well {
    fn get_column(&self, column: isize) -> &HashMap<isize, bool> {
        &self.columns[TryInto::<usize>::try_into(column).unwrap()]
    }

    fn get_column_mut(&mut self, column: isize) -> &mut HashMap<isize, bool> {
        &mut self.columns[TryInto::<usize>::try_into(column).unwrap()]
    }

    fn max_height(&self) -> isize {
        self.columns
            .iter()
            .map(Self::max_column_height)
            .max()
            .unwrap_or(-1)
    }

    fn max_column_height(col: &HashMap<isize, bool>) -> isize {
        *col.keys().max().unwrap_or(&-1isize)
    }

    fn hit_something(&self, coord: Coord) -> bool {
        !(0..=6).contains(&coord.0)
            || coord.1 < 0
            || self.get_column(coord.0).contains_key(&coord.1)
    }

    fn rock_hit_something(&self, shape: RockShape, coord: Coord) -> bool {
        for rock_cell in shape.cells() {
            if self.hit_something((coord.0 + rock_cell.0, coord.1 + rock_cell.1)) {
                return true;
            }
        }
        false
    }

    fn paste_rock(&mut self, shape: RockShape, coord: Coord) {
        for rock_cell in shape.cells() {
            self.get_column_mut(coord.0 + rock_cell.0)
                .insert(coord.1 + rock_cell.1, true);
        }
    }

    fn drop_one(&mut self, gusts: &mut impl Iterator<Item = char>, rock: RockShape) {
        let mut rock_coord = (2, self.max_height() + 4);

        loop {
            let blow_offset = match gusts.next() {
                Some('<') => -1,
                Some('>') => 1,
                _ => unreachable!(),
            };

            let blown_rock_coord = (rock_coord.0 + blow_offset, rock_coord.1);
            if !self.rock_hit_something(rock, blown_rock_coord) {
                rock_coord = (blown_rock_coord.0, blown_rock_coord.1);
            }

            let rock_coord_below = (rock_coord.0, rock_coord.1 - 1);
            if self.rock_hit_something(rock, rock_coord_below) {
                self.paste_rock(rock, rock_coord);
                break;
            }

            rock_coord = rock_coord_below;
        }
    }
}

fn main() {
    let mut gusts = INPUTS.trim().chars().cycle().peekable();
    let rocks_dont_drop_pls = RockShape::all(); //i hate rust
    let mut rocks = rocks_dont_drop_pls
        .iter()
        .copied()
        .enumerate()
        .cycle()
        .peekable();
    let mut well = Well::default();

    let mut rock_index = 0;
    while rock_index < 2022 {
        rock_index += 1;
        well.drop_one(&mut gusts, rocks.next().unwrap().1);
    }

    let part1 = well.max_height() + 1;
    println!("part1: {}", part1);
}
