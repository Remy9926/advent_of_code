fn part_1(puzzle_input: &str) -> usize {
    let lines: Vec<_> = puzzle_input.trim().split("\n").collect();
    let mut maps: Vec<Vec<Map>> = Vec::new();
    let mut seeds: Vec<usize> = Vec::new();
    let seed_line: Vec<_> = lines[0].split(":").collect();
    let all_seeds = seed_line[1].trim();
    let mut num_maps = 0;

    for i in all_seeds.split(" ") {
        seeds.push(i.parse::<usize>().unwrap());
    }

    for line in &lines[1..] {
        if line.contains("map") {
            num_maps += 1;
            maps.push(Vec::new());
        }
        else if *line != "" {
            let nums: Vec<_> = line.split(" ").collect();
            let to = nums[0].parse::<usize>().unwrap();
            let from = nums[1].parse::<usize>().unwrap();
            let range = nums[2].parse::<usize>().unwrap();
            let mapping = Map { to: to, from: from, range: range };
            maps[num_maps - 1].push(mapping);
        }
    }
    
    let mut lowest_location_number = usize::MAX;
    for seed in seeds {
        lowest_location_number = cmp::min(check_seed(seed, &maps), lowest_location_number);
    }
    
    lowest_location_number
}

fn check_seed(seed: usize, maps: &Vec<Vec<Map>>) -> usize {
    let mut map_number = 0;
    let mut mapped_to = seed;
    while map_number != maps.len() {
        for map in &maps[map_number] {
            if map.is_in_range(mapped_to) {
                mapped_to = map.get_mapping(mapped_to);
                break;
            }
        }
        map_number += 1;
    }
    mapped_to
}

fn part_2(puzzle_input: &str) -> usize {
    let lines: Vec<_> = puzzle_input.trim().split("\n").collect();
    let mut maps: Vec<Vec<Map>> = Vec::new();
    let mut seed_ranges: Vec<SeedRange> = Vec::new();
    let seed_line: Vec<_> = lines[0].split(":").collect();
    let all_seeds = seed_line[1].trim();
    let mut num_maps = 0;
    
    let seeds: Vec<_> = all_seeds.split(" ").collect();

    for i in 0..(seeds.len() >> 1)  {
        let from = seeds[2 * i].parse::<usize>().unwrap();
        let range = seeds[2 * i + 1].parse::<usize>().unwrap();
        let seed_range = SeedRange { from: from, range: range };
        seed_ranges.push(seed_range);
    }
    for line in &lines[1..] {
        if line.contains("map") {
            num_maps += 1;
            maps.push(Vec::new());
        }
        else if *line != "" {
            let nums: Vec<_> = line.split(" ").collect();
            let to = nums[0].parse::<usize>().unwrap();
            let from = nums[1].parse::<usize>().unwrap();
            let range = nums[2].parse::<usize>().unwrap();
            let mapping = Map { to: to, from: from, range: range };
            maps[num_maps - 1].push(mapping);
        }
    }
    
    let mut lowest_location_number = usize::MAX;

    for i in 0..usize::MAX {
        let seed_number = check_seed_reverse(i, &maps);
        for range in &seed_ranges {
            if range.is_in_range(seed_number) {
                return i;
            }
        }
    }
}

fn check_seed_reverse(seed: usize, maps: &Vec<Vec<Map>>) -> usize {
    let mut map_number = maps.len();
    let mut mapped_to = seed;
    while map_number > 0 {
        for map in &maps[map_number - 1] {
            if map.is_in_range_reverse(mapped_to) {
                mapped_to = map.reverse_mapping(mapped_to);
                break;
            }
        }
        map_number -= 1;
    }
    mapped_to
}