mod solutions;
use solutions::*;

fn main() {
    println!("Day 01");
    println!(
        "Part one: {}",
        day01::solution_part_one(include_str!("../inputs/day01.txt"))
    );
    println!(
        "Part two: {}",
        day01::solution_part_two(include_str!("../inputs/day01.txt"))
    );

    println!("Day 02");
    println!(
        "Part one: {}",
        day02::solution_part_one(include_str!("../inputs/day02.txt"))
    );
    println!(
        "Part two: {}",
        day02::solution_part_two(include_str!("../inputs/day02.txt"))
    );

    println!("Day 03");
    println!(
        "Part one: {}",
        day03::solution_part_one(include_str!("../inputs/day03.txt"))
    );
    println!(
        "Part two: {}",
        day03::solution_part_two(include_str!("../inputs/day03.txt"))
    );
}
