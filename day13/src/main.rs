use serde_json::Value;
use std::cmp::Ordering;

// const INPUTS: &str = r#"
// [1,1,3,1,1]
// [1,1,10,1,1]
//
// [[1],[2,3,4]]
// [[1],4]
//
// [9]
// [[8,7,6]]
//
// [[4,4],4,4]
// [[4,4],4,4,4]
//
// [7,7,7,7]
// [7,7,7]
//
// []
// [3]
//
// [[[]]]
// [[]]
//
// [1,[2,[3,[4,[5,6,7]]]],8,9]
// [1,[2,[3,[4,[5,6,0]]]],8,9]
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let part1 = INPUTS
        .trim()
        .split("\n\n")
        .map(|packets| {
            let (left, right) = packets.split_once("\n").unwrap();
            (
                serde_json::from_str::<Value>(left).unwrap(),
                serde_json::from_str::<Value>(right).unwrap(),
            )
        })
        .enumerate()
        .filter(|(_idx, (l, r))| match check_pair(l, r) {
            Ordering::Greater => false,
            _ => true,
        })
        .map(|(idx, _)| idx as i32 + 1)
        .sum::<i32>();

    println!("part1: {:?}", part1);
}

fn check_pair(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(x), Value::Number(y)) => x.as_u64().unwrap().cmp(&y.as_u64().unwrap()),
        (Value::Array(a), Value::Array(b)) => {
            for i in 0..a.len().max(b.len()) {
                match (a.get(i), b.get(i)) {
                    (None, _) => return Ordering::Less,
                    (_, None) => return Ordering::Greater,
                    (Some(x), Some(y)) => match check_pair(x, y) {
                        Ordering::Equal => {}
                        c => return c,
                    },
                }
            }
            Ordering::Equal
        }
        (Value::Array(_), Value::Number(_)) => check_pair(left, &Value::Array(vec![right.clone()])),
        (Value::Number(_), Value::Array(_)) => check_pair(&Value::Array(vec![left.clone()]), right),
        _ => unreachable!(),
    }
}
