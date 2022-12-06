use std::collections::HashSet;
use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let input_string =
        fs::read_to_string(&args[0]).expect("Something went wrong reading the file!");

    if let Some(result) = find_marker_position(&input_string, 4) {
        println!("Result of puzzle 1: {result}");
    }

    Ok(())
}

fn find_marker_position(s: &str, sequence_size: usize) -> Option<usize> {
    let chars: Vec<_> = s.chars().collect();
    let iter = chars.windows(sequence_size);

    for (i, sequence) in iter.enumerate() {
        let set: HashSet<_> = sequence.iter().collect();

        if set.len() == sequence_size {
            return Some(i + sequence_size);
        }
    }

    None
}
