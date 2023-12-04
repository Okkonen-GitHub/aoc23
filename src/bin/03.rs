use std::fs::read_to_string;
use std::rc::Rc;

const VALID_NUMS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
const NON_SYMBOL: [char; 11] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'];

// returns starting and ending indexes for a line and parses the number
fn find_num(line: &str) -> Vec<(usize, usize, u32)> {
    let mut indexes = vec![];
    let mut i = 0;
    let original = line;
    let mut line = line.chars();
    loop {
        let ch = match line.next() {
            Some(ch) => ch,
            None => break,
        };
        if VALID_NUMS.contains(&ch) {
            let start = i;
            let mut end = i;
            loop {
                if let Some(next) = line.next() {
                    if VALID_NUMS.contains(&next) {
                        i += 1;
                        end += 1;
                    } else {
                        i += 1;
                        break;
                    }
                }
            }
            let num = original.get(start..end + 1).unwrap().parse().unwrap();
            indexes.push((start, end, num));
        }
        i += 1;
    }
    indexes
}

fn part1(input: Rc<String>) -> u32 {
    let valid_nums = vec![];
    let lines = input.lines();
    for (i, line) in lines.clone().enumerate().take(2) {
        let indexes = find_num(line);
        for (start, end, num) in indexes {
            if i > 0 {
                let over = if let Some(line_above) = lines.clone().nth(i - 1) {
                    line_above.get(start - 1..end + 2)
                } else {
                    None
                };
                dbg!(over, num);
            }
        }
    }
    valid_nums.iter().sum()
}

fn main() {
    let input = Rc::new(read_to_string("./inputs/03.in").unwrap());
    let p1 = part1(Rc::clone(&input));

    println!("{p1}");
}
