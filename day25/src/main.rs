// const INPUTS: &str = r#"
// 1=-0-2
// 12111
// 2=0=
// 21
// 2=01
// 111
// 20012
// 112
// 1=-1=
// 1-12
// 12
// 1=
// 122
// "#;
const INPUTS: &str = include_str!("../input.txt");

fn main() {
    let mut total = 0_i64;
    for line in INPUTS.trim().lines() {
        let mut coef = 1;
        for x in line.chars().rev() {
            let k = match x {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!(),
            } * coef;

            total += k;
            coef *= 5;
        }
    }
    println!("total: {}", total);

    let mut output = String::new();
    while total != 0 {
        let rem = total % 5;
        total /= 5;

        if rem <= 2 {
            output = rem.to_string() + &output;
        } else {
            output = match rem {
                3 => "=",
                4 => "-",
                _ => "",
            }.to_owned() + &output;
            total += 1;
        }
    }

    println!("part1: {}", output);
}
