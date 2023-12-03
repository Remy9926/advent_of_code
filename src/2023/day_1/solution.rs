use dotenv::dotenv;
use std::collections::HashMap;

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