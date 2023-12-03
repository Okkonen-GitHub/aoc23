use std::fs::read_to_string;
use std::rc::Rc;

fn part1(input: Rc<String>) -> u32 {
    let valid_nums = vec![];
    for line in input.lines() {}
    valid_nums.iter().sum()
}

fn main() {
    let input = Rc::new(read_to_string("./inputs/03.in").unwrap());
    let p1 = part1(Rc::clone(&input));

    println!("{p1}");
}
