use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input").unwrap();
    let mut nums = vec![];
    for line in input.lines() {
        let mut num = 0;
        'inner: for c in line.chars() {
            match c.to_digit(10) {
                Some(n) => {
                    num += 10 * n;
                    break 'inner;
                }
                None => {}
            }
        }
        'inner: for c in line.chars().rev() {
            match c.to_digit(10) {
                Some(n) => {
                    num += n;
                    break 'inner;
                }
                None => {}
            }
        }
        nums.push(num);
    }
    let sum: u32 = nums.iter().sum();
    println!("{sum}");
}
