use std::collections::HashMap;

fn part1(input: String) -> u32 {
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

fn main() {
    let input = std::fs::read_to_string("./inputs/02.in").unwrap();
    println!("p1: {}", part1(input));
}
