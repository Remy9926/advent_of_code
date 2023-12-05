fn part_1(puzzle_input: &str) -> i64 {
    let lines: Vec<_> = puzzle_input.trim().split("\n").collect();
    let mut total_points = 0;
    let mut curr_points = 0;
    
    for i in 0..lines.len() {
        let line = lines[i];
        let card: Vec<_> = line.split(":").collect();
        let numbers = card[1].trim();
        let mut winning_nums: Vec<String> = Vec::new();
        let mut curr_num = String::from("");
        let chars: Vec<_> = numbers.chars().collect();
        let mut have_numbers = 0;

        for j in 0..chars.len() {
            let char = chars[j];
            if char == '|' {
                have_numbers = j + 2;
                break;
            }
            if char.is_numeric() {
                curr_num += &(char).to_string();
            }
            else if curr_num != "" {
                winning_nums.push(curr_num.clone());
                curr_num = String::from("");
                }
        }

        for j in have_numbers..chars.len() {
            let char = chars[j];
            if char.is_numeric() {
                curr_num += &(char).to_string();
            }
            if !char.is_numeric() || j == chars.len() - 1 {
                if winning_nums.contains(&curr_num) {
                    curr_points += 1;
                }
                curr_num = String::from("");
            }
        }
        if curr_points > 0 {
            total_points += 2_i64.checked_pow(curr_points - 1).unwrap();
        }
        curr_points = 0;
    }
    total_points
}

fn part_2(puzzle_input: &str) -> i64 {
    let lines: Vec<_> = puzzle_input.trim().split("\n").collect();
    let mut total_points = 0;
    let mut curr_points = 0;
    let mut copies: HashMap<usize, usize> = HashMap::new();
    
    for i in 0..lines.len() {
        let line = lines[i];
        println!("{}", line);
        let card: Vec<_> = line.split(":").collect();
        let numbers = card[1].trim();
        let mut winning_nums: Vec<String> = Vec::new();
        let mut curr_num = String::from("");
        let chars: Vec<_> = numbers.chars().collect();
        let mut have_numbers = 0;

        for j in 0..chars.len() {
            let char = chars[j];
            if char == '|' {
                have_numbers = j + 2;
                break;
            }
            if char.is_numeric() {
                curr_num += &(char).to_string();
            }
            else if curr_num != "" {
                winning_nums.push(curr_num.clone());
                curr_num = String::from("");
                }
        }

        for j in have_numbers..chars.len() {
            let char = chars[j];
            if char.is_numeric() {
                curr_num += &(char).to_string();
            }
            if !char.is_numeric() || j == chars.len() - 1 {
                if winning_nums.contains(&curr_num) {
                    curr_points += 1;
                }
                curr_num = String::from("");
            }
        }
        if !copies.contains_key(&i) {
            copies.insert(i, 0);
        }
        let num_times = *copies.get_mut(&i).unwrap() + 1;

        for j in i + 1..i + curr_points + 1 {
            if !copies.contains_key(&j) {
                copies.insert(j, 0);
            }
            let copy = copies.get_mut(&j).unwrap();
            *copy += num_times;
        }
        curr_points = 0;
    }
    
    total_points += copies.len();
    for key in copies.keys() {
        total_points += copies.get(key).unwrap();
    }
    
    total_points
}