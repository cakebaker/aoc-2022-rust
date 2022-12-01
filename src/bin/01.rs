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

    for line in lines {
        if line.is_empty() {
            if calories_carried_by_elf > max_calories_carried {
                max_calories_carried = calories_carried_by_elf;
            }

            calories_carried_by_elf = 0;
        } else {
            calories_carried_by_elf += line.parse::<u64>().unwrap();
        }
    }

    println!("Result of puzzle 1: {max_calories_carried}");

    Ok(())
}
