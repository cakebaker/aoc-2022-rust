use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let file_content =
        fs::read_to_string(&args[0]).expect("Something went wrong reading the file!");
    let lines = file_content.lines();

    let mut max_calories_carried = 0;
    let mut calories_carried_by_elf = 0;
    let mut all_calories_carried = Vec::new();

    for line in lines {
        if line.is_empty() {
            if calories_carried_by_elf > max_calories_carried {
                max_calories_carried = calories_carried_by_elf;
            }

            all_calories_carried.push(calories_carried_by_elf);
            calories_carried_by_elf = 0;
        } else {
            calories_carried_by_elf += line.parse::<u64>().unwrap();
        }
    }

    all_calories_carried.push(calories_carried_by_elf);
    all_calories_carried.sort_unstable();
    let sum_top_3: u64 = all_calories_carried.iter().rev().take(3).sum();

    println!("Result of puzzle 1: {max_calories_carried}");
    println!("Result of puzzle 2: {sum_top_3}");

    Ok(())
}
