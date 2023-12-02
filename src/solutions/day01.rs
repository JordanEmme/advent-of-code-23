use std::thread::current;

pub fn solution(input: &str) -> u16 {
    let mut sum: u16 = 0u16;
    input.lines().for_each( |line|
        {
            sum += find_line_number(line);
        }
    );
    return sum;
}

fn find_line_number(line: &str) -> u16{
    let characters: Vec<char> = line.chars().collect();
    let mut ten: u16 = 0u16;
    let mut unit: u16 = 0u16;
    for i in 0usize..characters.len() {
        let current_char = characters[i];
        if current_char.is_numeric() {
            ten = 10 * current_char.to_string().parse::<u16>().unwrap();
            break;
        }
    }
    for i in (0usize..characters.len()).rev(){
        let current_char = characters[i];
        if current_char.is_numeric() {
            unit = current_char.to_string().parse().unwrap();
            break;
        }
    }
    return ten + unit;
}
