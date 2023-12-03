use dotenv::dotenv;
use std::collections::HashMap;

async fn main() {
    dotenv().ok();

    let aoc_token = std::env::var("AOC_TOKEN").expect("Please set a valid AOC token");
    let input = get_input_2023(1, &aoc_token).await;
    let solution_1 = part_1(&input);
    let solution_2 = part_2(&input);
    println!("Part 1: {}\n Part 2: {}", solution_1, solution_2);
}

fn part_1(puzzle_input: &str) -> i64 {
    let lines: Vec<_> = puzzle_input.trim().split("\n").collect();
    let mut total = 0;
    for i in 0..lines.len() {
        let mut number = "".to_owned();
        let line = lines[i];
        let chars: Vec<_> = line.chars().collect();

        for j in 0..chars.len() {
            let char = chars[j];
            if char.is_numeric() {
                number += &char.to_string();
                break;
            }
        }
        for j in (0..chars.len()).rev() {
            let char = chars[j];
            if char.is_numeric() {
                number += &char.to_string();
                break;
            }
        }
        total += number.parse::<i64>().unwrap();
    }
    total
}

fn part_2(puzzle_input: &str) -> i64 {
    let lines: Vec<_> = puzzle_input.trim().split("\n").collect();
    let mut nums = HashMap::<&str, &str>::new();
    let mut total = 0;
    let strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    for i in 0..strings.len() {
        nums.insert(&strings[i], &digits[i]);
    }

    for i in 0..lines.len() {
        let mut number = "".to_owned();
        let line = lines[i];
        let chars: Vec<_> = line.chars().collect();
        let mut num_string = "".to_owned();
        let mut string_key = "".to_owned();

        for j in 0..chars.len() {
            let char = chars[j];
            if char.is_numeric() {
                number += &char.to_string();
                break;
            }
            else {
                num_string += &char.to_string();
                for k in 0..strings.len() {
                    let key = strings[k];
                    if num_string.contains(&key) {
                        string_key = key.to_string();
                        break;
                    }
                }
                if string_key != "" {
                    number += nums.get(string_key.as_str()).unwrap().to_owned();
                    break;
                }
            }
        }
        
        num_string = String::from("");
        string_key = "".to_owned();
        for j in (0..chars.len()).rev() {
            let char = chars[j];
            if char.is_numeric() {
                number += &char.to_string();
                break;
            }
            else {
                num_string = char.to_string() + &num_string;
                for k in 0..strings.len() {
                    let key = strings[k];
                    if num_string.contains(&key) {
                        string_key = key.to_string();
                        break;
                    }
                }
                if string_key != "" {
                    number += nums.get(string_key.as_str()).unwrap().to_owned();
                    break;
                }
            }
        }
        total += number.parse::<i64>().unwrap();
    }
    total
}