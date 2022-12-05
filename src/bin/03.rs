use std::collections::HashSet;
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

    let mut total_priorities = 0;

    for line in lines {
        let (first, second) = line.split_at(line.len() / 2);

        let first = first.chars().collect::<HashSet<_>>();
        let second = second.chars().collect::<HashSet<_>>();

        let shared = *first.intersection(&second).next().unwrap();

        if let Some(priority) = priority(shared) {
            total_priorities += priority;
        }
    }

    println!("Result of puzzle 1: {total_priorities}");

    Ok(())
}

fn priority(ch: char) -> Option<usize> {
    match ch {
        'a'..='z' => Some(ch as usize - 96),
        'A'..='Z' => Some(ch as usize - 38),
        _ => None,
    }
}
