pub fn solution_part_one(input: &str) -> u16 {
    let mut sum: u16 = 0u16;
    input.lines().for_each(|line| {
        sum += find_line_number(line);
    });
    return sum;
}

pub fn solution_part_two(input: &str) -> u16 {
    let mut sum: u16 = 0u16;
    input.lines().for_each(|line| {
        let rewritten_line = rewrite(line);
        sum += find_line_number(&*rewritten_line);
    });
    return sum;
}

fn rewrite(original_line: &str) -> String {
    let mut rewritten_line: String;
    rewritten_line = original_line.replace("zero", "0");
    rewritten_line = rewritten_line.replace("one", "o1e"); // because of twone and oneight
    rewritten_line = rewritten_line.replace("two", "t2"); //because of eightwo
    rewritten_line = rewritten_line.replace("three", "t3e"); // because of eighthree and threeight
    rewritten_line = rewritten_line.replace("four", "4");
    rewritten_line = rewritten_line.replace("five", "5e"); // because of fiveeight
    rewritten_line = rewritten_line.replace("six", "6");
    rewritten_line = rewritten_line.replace("seven", "7n"); // because of sevenine
    rewritten_line = rewritten_line.replace("eight", "e8"); // because of nineight
    rewritten_line = rewritten_line.replace("nine", "9");
    return rewritten_line;
}

fn find_line_number(line: &str) -> u16 {
    let characters: Vec<char> = line.chars().collect();
    let mut ten: u16 = 0u16;
    let mut unit: u16 = 0u16;
    let mut found_ten: bool = false;
    let mut found_unit: bool = false;
    let mut offset: usize = 0usize;
    while (!found_ten || !found_unit) && offset < characters.len() {
        let left_char = characters[offset];
        let right_char = characters[characters.len() - offset - 1];
        if left_char.is_numeric() && !found_ten {
            ten = 10 * left_char.to_string().parse::<u16>().unwrap();
            found_ten = true;
        }
        if right_char.is_numeric() && !found_unit{
            unit = right_char.to_string().parse::<u16>().unwrap();
            found_unit = true;
        }
        offset += 1;
    }
    return ten + unit;
}
