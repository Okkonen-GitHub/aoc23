use std::fs::read_to_string;

fn part1(input: String) -> u32 {
    let mut nums = vec![];
    for line in input.lines() {
        let mut num = 0;
        for c in line.chars() {
            match c.to_digit(10) {
                Some(n) => {
                    num += 10 * n;
                    break;
                }
                None => {}
            }
        }
        for c in line.chars().rev() {
            match c.to_digit(10) {
                Some(n) => {
                    num += n;
                    break;
                }
                None => {}
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
                match &line.get(a..a + b) {
                    Some(sub) => {
                        match sub.chars().next().unwrap().to_digit(10) {
                            Some(n) => {
                                num += 10 * n;
                                // dbg!(num, line);
                                break 'outer;
                            }
                            None => {}
                        }

                        let parsed = to_num(sub);
                        if parsed != 0 {
                            num += 10 * parsed;
                            // dbg!(num, sub, line);
                            break 'outer;
                        }
                    }
                    None => {}
                }
            }
        }
        'outer: for a in 0..len {
            for b in 1..=5 {
                match &line.chars().rev().collect::<String>().get(a..b + a) {
                    Some(sub) => {
                        match sub.chars().next().unwrap().to_digit(10) {
                            Some(n) => {
                                num += n;
                                // dbg!(num, line);
                                break 'outer;
                            }
                            None => {}
                        }
                        let sub = sub.chars().to_owned().rev().collect::<String>();
                        let parsed = to_num(&sub);
                        if parsed != 0 {
                            num += parsed;
                            // dbg!(num, sub, line);
                            break 'outer;
                        }
                    }
                    None => {}
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
