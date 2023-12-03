use std::cmp;

fn part_1(puzzle_input: &str) -> i64 {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut sum = 0;
    
    for i in 0..lines.len() {
        let line = lines[i].trim();
        let details: Vec<_> = line.split(":").collect();
        let game_detail: Vec<_> = details[0].split(" ").collect();
        let game_number = game_detail[1].parse::<i64>().unwrap();
        let outcomes = details[1].trim();
        let rounds: Vec<_> = outcomes.split(";").collect();
        let mut valid_round = true;

        for j in 0..rounds.len() {
            let round = rounds[j];
            let round_details: Vec<_> = round.split(",").collect();
            
            for k in 0..round_details.len() {
                let detail = round_details[k].trim();
                let num_and_color: Vec<_> = detail.split(" ").collect();
                let num = num_and_color[0].parse::<i64>().unwrap();
                let color = num_and_color[1];
                if color == "red" && num > 12 { valid_round = false; }
                if color == "green" && num > 13 { valid_round = false; }
                if color == "blue" && num > 14 { valid_round = false; }
            }
            if !valid_round {
                break;
            }
        }
        if valid_round {
            sum += game_number;
        }
    }
    sum
}

fn part_2(puzzle_input: &str) -> i64 {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut sum = 0;
    
    for i in 0..lines.len() {
        let line = lines[i].trim();
        let details: Vec<_> = line.split(":").collect();
        let game_detail: Vec<_> = details[0].split(" ").collect();
        let outcomes = details[1].trim();
        let rounds: Vec<_> = outcomes.split(";").collect();
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for j in 0..rounds.len() {
            let round = rounds[j];
            let round_details: Vec<_> = round.split(",").collect();
            
            for k in 0..round_details.len() {
                let detail = round_details[k].trim();
                let num_and_color: Vec<_> = detail.split(" ").collect();
                let num = num_and_color[0].parse::<i64>().unwrap();
                let color = num_and_color[1];
                if color == "red" {
                    min_red = cmp::max(min_red, num);
                }
                if color == "green" {
                    min_green = cmp::max(min_green, num);
                }
                if color == "blue" {
                    min_blue = cmp::max(min_blue, num);
                }
            }
        }
        let power = min_red * min_green * min_blue;
        sum += power;
    }
    sum
}