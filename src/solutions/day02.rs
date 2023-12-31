const RED_MAX: u16 = 12u16;
const GREEN_MAX: u16 = 13u16;
const BLUE_MAX: u16 = 14u16;

pub fn solution_part_one(input: &str) -> u16 {
    // max is potentially 4,950 => need u16
    let mut result: u16 = 0u16;
    for (line_num, line) in input.lines().enumerate() {
        result += if game_is_valid(line) {
            line_num as u16 + 1u16
        } else {
            0
        };
    }
    return result;
}

pub fn solution_part_two(input: &str) -> u32 {
    return input
        .lines()
        .fold(0u32, |sum, line| sum + triplet_prod(min_cube_set(line)))
}

fn game_is_valid(game_line: &str) -> bool {
    let words_vector = game_line.split(" ").collect::<Vec<&str>>();
    for i in (2..(words_vector.len() - 1)).step_by(2) {
        let num = words_vector[i].parse::<u16>().unwrap();
        let col = words_vector[i + 1];
        if col.starts_with("red") && num > RED_MAX {
            return false;
        } else if col.starts_with("blue") && num > BLUE_MAX {
            return false;
        } else if col.starts_with("green") && num > GREEN_MAX {
            return false;
        }
    }
    return true;
}

fn min_cube_set(game_line: &str) -> (u32, u32, u32) {
    let words_vector = game_line.split(" ").collect::<Vec<&str>>();
    let mut min_red: u32 = 0u32;
    let mut min_green: u32 = 0u32;
    let mut min_blue: u32 = 0u32;
    for i in (2..(words_vector.len() - 1)).step_by(2) {
        let num = words_vector[i].parse::<u32>().unwrap();
        let col = words_vector[i + 1];
        if col.starts_with("red") && num > min_red {
            min_red = num;
        } else if col.starts_with("blue") && num > min_blue {
            min_blue = num;
        } else if col.starts_with("green") && num > min_green {
            min_green = num;
        }
    }
    return (min_red, min_green, min_blue);
}

fn triplet_prod(triple: (u32, u32, u32)) -> u32 {
    triple.0 * triple.1 * triple.2
}
