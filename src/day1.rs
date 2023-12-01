use regex::Regex;
use std::fs;

fn extract_numbers(s: &str) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    let ns: Vec<&str> = re.find_iter(s).map(|m| m.as_str()).collect();
    let n_str = ns.join("");
    let mut result = String::from(&n_str[0..1].to_owned());
    result.push_str(&n_str[n_str.len() - 1..]);
    result.parse::<i32>().unwrap()
}

fn extract_numbers_2(s: &str) -> i32 {
    // repeat the initial and ending letters to prevent missing words that shares
    // the same character: ex.: twone = t2one = t2oo1e = 21
    let mut ss = s.replace("one", "o1e");
    ss = ss.replace("two", "t2o");
    ss = ss.replace("three", "t3e");
    ss = ss.replace("four", "f4r");
    ss = ss.replace("five", "f5e");
    ss = ss.replace("six", "s6x");
    ss = ss.replace("seven", "s7n");
    ss = ss.replace("eight", "e8t");
    ss = ss.replace("nine", "n9e");
    extract_numbers(ss.as_str())
}

pub fn solution() {
    let file_path = "src/day1.txt";
    let lines: Vec<(i32, i32)> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|s| (extract_numbers(s), extract_numbers_2(s)))
        .collect();
    let mut r1 = 0;
    let mut r2 = 0;
    for line in lines {
        r1 += line.0;
        r2 += line.1;
    }
    println!("Day 1");
    println!("Solution 1: {r1}");
    println!("Solution 2: {r2}");
}
