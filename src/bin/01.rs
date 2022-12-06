use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let file_content =
        fs::read_to_string(&args[0]).expect("Something went wrong reading the file!");

    let max_calories_carried = file_content
        .split("\n\n")
        .map(calories_carried_by_elf)
        .max()
        .unwrap();

    let mut all_calories_carried: Vec<_> = file_content
        .split("\n\n")
        .map(calories_carried_by_elf)
        .collect();

    all_calories_carried.sort_unstable();
    all_calories_carried.reverse();

    let sum_top_3: usize = all_calories_carried.iter().take(3).sum();

    println!("Result of puzzle 1: {max_calories_carried}");
    println!("Result of puzzle 2: {sum_top_3}");

    Ok(())
}

fn calories_carried_by_elf(s: &str) -> usize {
    s.lines().map(|l| l.parse::<usize>().unwrap()).sum()
}
