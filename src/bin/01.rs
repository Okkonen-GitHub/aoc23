use std::fs::read_to_string;

fn part1(input: String) -> u32 {
    let mut nums = vec![];
    for line in input.lines() {
        let mut num = 0;
        for c in line.chars() {
            if let Some(n) = c.to_digit(10) {
                num += 10 * n;
                break;
            }
        }
        for c in line.chars().rev() {
            if let Some(n) = c.to_digit(10) {
                num += n;
                break;
            }
        }
        nums.push(num);
    }
    nums.iter().sum()
}

fn part2(input: String) -> u32 {
    fn to_num(it: &str) -> u32 {
        match it {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        }
    }
    let mut nums = vec![];
    for line in input.lines() {
        let mut num = 0;
        let len = line.len();
        'outer: for a in 0..len {
            for b in 1..=5 {
                if let Some(sub) = &line.get(a..a + b) {
                    if let Some(n) = sub.chars().next().unwrap().to_digit(10) {
                        num += 10 * n;
                        // dbg!(num, line);
                        break 'outer;
                    }

                    let parsed = to_num(sub);
                    if parsed != 0 {
                        num += 10 * parsed;
                        // dbg!(num, sub, line);
                        break 'outer;
                    }
                }
            }
        }
        'outer: for a in 0..len {
            for b in 1..=5 {
                if let Some(sub) = &line.chars().rev().collect::<String>().get(a..b + a) {
                    if let Some(n) = sub.chars().next().unwrap().to_digit(10) {
                        num += n;
                        // dbg!(num, line);
                        break 'outer;
                    }
                    let sub = sub.chars().to_owned().rev().collect::<String>();
                    let parsed = to_num(&sub);
                    if parsed != 0 {
                        num += parsed;
                        // dbg!(num, sub, line);
                        break 'outer;
                    }
                }
            }
        }
        nums.push(num);
    }
    nums.iter().sum()
}

fn main() {
    let input = read_to_string("./inputs/01.in").unwrap();
    let p1 = part1(input.clone());
    let p2 = part2(input);

    println!("{p1}");
    println!("{p2}");
}
