use regex::Regex;
use std::fs;

fn is_valid(color: &str, qtd: i32) -> bool {
    match color {
        "red" => qtd <= 12,
        "green" => qtd <= 13,
        "blue" => qtd <= 14,
        _ => false,
    }
}

fn read_line(s: &str) -> i32 {
    let mut parts = s.split(":");
    let game_str = parts.next().unwrap();
    let re = Regex::new(r"\d+").unwrap();
    let game_number = re.find(game_str).unwrap().as_str().parse::<i32>().unwrap();
    let sets_str = parts.next().unwrap();
    let sets: Vec<&str> = sets_str.split(";").collect();
    for set in sets {
        let groups: Vec<&str> = set.split(",").collect();
        for group in groups {
            let re = Regex::new(r"(\d+)\s([a-z]+)").unwrap();
            let caps = re.captures(group).unwrap();
            let qtd = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let color = caps.get(2).unwrap().as_str();
            if !is_valid(color, qtd) {
                return 0;
            }
        }
    }
    game_number
}

fn power_of_fewer(line: &str) -> i32 {
    let mut parts = line.split(":");
    let _ = parts.next();
    let sets_str = parts.next().unwrap();
    let sets: Vec<&str> = sets_str.split(";").collect();
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for set in sets {
        let groups: Vec<&str> = set.split(",").collect();
        for group in groups {
            let re = Regex::new(r"(\d+)\s([a-z]+)").unwrap();
            let caps = re.captures(group).unwrap();
            let qtd = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let color = caps.get(2).unwrap().as_str();
            match color {
                "red" => {
                    if qtd > max_red {
                        max_red = qtd;
                    }
                }
                "green" => {
                    if qtd > max_green {
                        max_green = qtd;
                    }
                }
                "blue" => {
                    if qtd > max_blue {
                        max_blue = qtd;
                    }
                }
                _ => {}
            }
        }
    }
    max_red * max_green * max_blue
}

pub fn solution() {
    let file_path = "src/day2.txt";
    let s1: i32 = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(read_line)
        .sum();
    let s2: i32 = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(power_of_fewer)
        .sum();
    println!("Solution 1: {}", s1);
    println!("Solution 2: {}", s2);
}
