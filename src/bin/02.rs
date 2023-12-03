use std::collections::HashMap;
use std::fs::read_to_string;
use std::rc::Rc;

fn part1(input: Rc<String>) -> u32 {
    let mut max_map: HashMap<&str, u32> = HashMap::new();
    max_map.insert("green", 13);
    max_map.insert("blue", 14);
    max_map.insert("red", 12);
    let mut pass = 0;
    'outer: for line in input.lines().take(102) {
        for rounds in line.split(":").last().unwrap().split(';') {
            for parts in rounds.split(',') {
                let mut splt = parts.split(' ').skip(1);
                let (num, colour) = (splt.next().unwrap(), splt.next().unwrap());
                if num.trim().parse::<u32>().unwrap() > *max_map.get(colour).unwrap() {
                    continue 'outer;
                }
            }
        }
        let mut game_id = line.split(" ").skip(1).next().unwrap().split(":");
        let game_id = game_id.next().unwrap().parse::<u32>().unwrap();
        pass += game_id;
    }
    pass
}

fn part2(input: Rc<String>) -> u32 {
    let mut nums = vec![];
    for line in input.lines() {
        let mut greens = vec![];
        let mut blues = vec![];
        let mut reds = vec![];
        for rounds in line.split(":").last().unwrap().split(";") {
            for parts in rounds.split(",") {
                let mut splt = parts.split(" ").skip(1);
                let num = splt.next().unwrap().trim().parse::<u32>().unwrap();
                let colour = splt.next().unwrap();
                match colour {
                    "green" => {
                        greens.push(num);
                    }
                    "red" => {
                        reds.push(num);
                    }
                    "blue" => {
                        blues.push(num);
                    }
                    _ => unreachable!(),
                }
            }
        }
        nums.push(
            greens.iter().max().unwrap() * blues.iter().max().unwrap() * reds.iter().max().unwrap(),
        )
    }
    nums.iter().sum()
}

fn main() {
    let input = read_to_string("./inputs/02.in").unwrap();
    let input = Rc::new(input);
    let p1 = part1(Rc::clone(&input));
    let p2 = part2(Rc::clone(&input));
    println!("p1: {p1}, p2: {p2}");
}
