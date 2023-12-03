fn part_1(puzzle_input: &str) -> i64 {
    let mut char_grid: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;    
    let lines: Vec<_> = puzzle_input.trim().split("\n").collect();
    for i in 0..lines.len() {
        let line: Vec<_> = lines[i].chars().collect();
        char_grid.push(line);
    }
    
    let mut curr_number = String::from("");
    for row in 0..char_grid.len() {
        for col in 0..char_grid[row].len() {
            let char = char_grid[row][col];
            if char.is_numeric() {
                curr_number += &(char).to_string()
            }
            if col == char_grid[row].len() - 1 || !char.is_numeric() {
                if adjacent_to_symbol(&char_grid, row, col - curr_number.len(), curr_number.len()) {
                    sum += curr_number.parse::<i64>().unwrap();
                }
                curr_number = String::from("");
            }
        }
    }
    sum
}

fn part_2(puzzle_input: &str) -> i64 {
    let lines: Vec<_> = puzzle_input.trim().split("\n").collect();
    let mut sum = 0;
    let mut char_grid: Vec<Vec<char>> = Vec::new();
    let mut rows: Vec<HashMap<usize, i64>> = Vec::new();
    for i in 0..lines.len() {
        let line = lines[i];
        let line_chars: Vec<_> = line.chars().collect();
        let row_info = get_numbers_in_row(&line);

        char_grid.push(line_chars);
        rows.push(row_info);
    }
    for row in 0..char_grid.len() {
        for col in 0..char_grid[row].len() {
            let char = char_grid[row][col];
            if char == '*' {
                let (num_1, num_2) = check_adjacent(&char_grid, &rows, row, col);
                sum += num_1 * num_2;
            }
        }
    }
    sum
}

/// Searches for all adjacent neighbors to a number in the grid to find out if the number is
/// adjacent to any symbols
/// 
/// # Arguments
/// 
/// * `grid` - The grid of characters to search through
/// * `row` - The row in the grid to search through
/// * `start_y` - The starting y-coordinate in the grid to search through
/// * `length` - The length of the number to find adjacent symbols for
/// 
/// # Return
/// 
/// true if the number is adjacent to a symbol in the grid, otherwise false
fn adjacent_to_symbol(grid: &Vec<Vec<char>>, row: usize, start_y: usize, length: usize) -> bool {
    for col in start_y..start_y + length {
        if row > 0 {
            if !grid[row - 1][col].is_numeric() && grid[row - 1][col] != '.' {
                return true;
            }
            if col > 0 {
                if !grid[row - 1][col - 1].is_numeric() && grid[row - 1][col - 1] != '.' {
                    return true;
                }
            }
            if col < grid[row].len() - 1 {
                if !grid[row - 1][col + 1].is_numeric() && grid[row - 1][col + 1] != '.' {
                    return true;
                }
            }
        }
        if col > 0 {
            if !grid[row][col - 1].is_numeric() && grid[row][col - 1] != '.' {
                return true;
            }
        }
        if col < grid[row].len() - 1 {
            if !grid[row][col + 1].is_numeric() && grid[row][col + 1] != '.' {
                return true;
            }
        }
        if row < grid.len() - 1 {
            if !grid[row + 1][col].is_numeric() && grid[row + 1][col] != '.' {
                return true;
            }
            if col > 0 {
                if !grid[row + 1][col - 1].is_numeric() && grid[row + 1][col - 1] != '.' {
                    return true;
                }
            }
            if col < grid[row].len() - 1 {
                if !grid[row + 1][col + 1].is_numeric() && grid[row + 1][col + 1] != '.' {
                    return true;
                }
            }
        }
    }
    false
}

/// Given a specific coordinate in a grid of chars, searches all spaces in the matrix adjacent to
/// the coordinate and returns any adjacent numbers in the grid
/// 
/// # Arguments
/// * `grid` - A grid of characters
/// * `rows` - A vector of hashmaps indicating what number is at what coordinate in the grid
/// * `row` - The row coordinate of the coordinate to be checked
/// * `col` - The column coordinate of the coordinate to be checked
/// 
/// # Return
/// 
/// A tuple of two integers, which are the numbers adjacent to the coordinate in the grid. If there
/// are no adjacent numbers, or only 1 adjacent number, then 0 will be returned inside of the tuple
/// instead
fn check_adjacent(grid: &Vec<Vec<char>>, rows: &Vec<HashMap<usize, i64>>, row: usize, col: usize) -> (i64, i64) {
    let mut seen: Vec<i64> = Vec::new();
    if row > 0 {
        if !rows[row - 1].get(&col).is_none() {
            if grid[row - 1][col].is_numeric() {
                let num = rows[row - 1].get(&col).unwrap();
                if !seen.contains(&num) {
                    seen.push(*num);
                }
            }
        }
        if col > 0 {
            if !rows[row - 1].get(&(col - 1)).is_none() {
                if grid[row - 1][col - 1].is_numeric() {
                    let num = rows[row - 1].get(&(col - 1)).unwrap();
                    if !seen.contains(&num) {
                        seen.push(*num);
                    }
                }
            }
        }
        if col < grid[row].len() - 1 {
            if !rows[row - 1].get(&(col + 1)).is_none() {
                if grid[row - 1][col + 1].is_numeric() {
                    let num = rows[row - 1].get(&(col + 1)).unwrap();
                    if !seen.contains(&num) {
                        seen.push(*num);
                    }
                }
            }
        }
    }
    if col > 0 {
        if !rows[row].get(&(col - 1)).is_none() {
            if grid[row][col - 1].is_numeric() {
                let num = rows[row].get(&(col - 1)).unwrap();
                if !seen.contains(&num) {
                    seen.push(*num);
                }
            }
        }
    }
    if col < grid[row].len() - 1 {
        if !rows[row].get(&(col + 1)).is_none() {
            if !(rows[row].get(&(col + 1)).is_none()) {
                let num = rows[row].get(&(col + 1)).unwrap();
                if !seen.contains(&num) {
                    seen.push(*num);
                }
            }
        }
    }
    if row < grid.len() - 1 {
        if !rows[row + 1].get(&col).is_none() {
            if grid[row + 1][col].is_numeric() {
                let num = rows[row + 1].get(&col).unwrap();
                if !seen.contains(&num) {
                    seen.push(*num);
                }
            }
        }
        if col > 0 {
            if !rows[row + 1].get(&(col - 1)).is_none() {
                if grid[row + 1][col - 1].is_numeric() {
                    let num = rows[row + 1].get(&(col - 1)).unwrap();
                    if !seen.contains(&num) {
                        seen.push(*num);
                    }
                }
            }
        }
        if col < grid[row].len() - 1 {
            if !rows[row + 1].get(&(col + 1)).is_none() {
                if grid[row + 1][col + 1].is_numeric() {
                    let num = rows[row + 1].get(&(col + 1)).unwrap();
                    if !seen.contains(&num) {
                        seen.push(*num);
                    }
                }
            }
        }
    }
    if seen.len() == 0 {
        return (0, 0);
    }
    if seen.len() == 1 {
        return (seen[0], 0);
    }
    return (seen[0], seen[1]);
}

/// Given a string as a row in a grid of coordinates, parses the string to find all the numbers
/// in the string and stores the number at each column inside of a HashMap
/// 
/// # Arguments
/// 
/// * `row` - The string that represents an entire row in a matrix of characters
/// 
/// # Returns
/// 
/// A HashMap mapping the column in the matrix to the number that is stored inside of that column
fn get_numbers_in_row(row: &str) -> HashMap<usize, i64> {
    let mut col_to_number: HashMap<usize, i64> = HashMap::new();
    let chars: Vec<char> = row.chars().collect();
    let mut curr_num = String::from("");
    for i in 0..chars.len() {
        let char = chars[i];
        if char.is_numeric() {
            curr_num += &(char).to_string();
        }
        if (i == chars.len() - 1 || !char.is_numeric()) && curr_num != "" {
            let num = curr_num.parse::<i64>().unwrap();
            for j in i - curr_num.len()..i + 1 {
                col_to_number.insert(j, num);
            }
            curr_num = String::from("");
        }
    }
    col_to_number
}