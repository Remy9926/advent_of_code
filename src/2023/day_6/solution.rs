fn part_1(puzzle_input: &str) -> usize {
    let lines: Vec<_> = puzzle_input.trim().split("\n").collect();
    let mut times: Vec<_> = Vec::new();
    let mut distances: Vec<_> = Vec::new();
    let mut count = 1;

    let time_line: Vec<_> = lines[0].split(" ").collect();
    let distance_line: Vec<_> = lines[1].split(" ").collect();
    for i in 0..time_line.len() {
        if time_line[i].parse::<usize>().is_ok() {
            times.push(time_line[i].parse::<usize>().unwrap());
        }
        if i < distance_line.len() && distance_line[i].parse::<usize>().is_ok() {
            distances.push(distance_line[i].parse::<usize>().unwrap());
        }
    }

    for i in 0..times.len() {
        let mut lower_bound = 0;
        let mut upper_bound = 0;
        for j in 1..times[i] {
            if is_record(times[i], distances[i], j) {
                lower_bound = j;
                break;
            }
        }
        for j in (1..times[i]).rev() {
            if is_record(times[i], distances[i], j) {
                upper_bound = j;
                break;
            }
        }
        println!("({}, {})", lower_bound, upper_bound);
        count *= (upper_bound - lower_bound + 1);
        println!("{}", count);
    }
    count
}

fn part_2(puzzle_input: &str) -> usize {
    let lines: Vec<_> = puzzle_input.trim().split("\n").collect();
    let mut time: usize;
    let mut distance: usize;

    let time_line: Vec<_> = lines[0].split(" ").collect();
    let distance_line: Vec<_> = lines[1].split(" ").collect();
    
    let mut parse_string = String::from("");
    for i in 1..time_line.len() {
        parse_string += time_line[i];
    }
    time = parse_string.parse::<usize>().unwrap();
    parse_string.clear();

    for i in 1..distance_line.len() {
        parse_string += distance_line[i];
    }
    distance = parse_string.parse::<usize>().unwrap();

    let mut lower_bound = 0;

    for i in 1..time + 1 {
        if is_record(time, distance, i) {
            lower_bound = i;
            break;
        }
    }
    time - (2 * lower_bound) + 1
}

fn is_record(time: usize, distance: usize, pressed_time: usize) -> bool {
    let time_remaining = time - pressed_time;
    let distance_traveled = pressed_time * time_remaining;
    distance < distance_traveled
}