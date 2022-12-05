use std::env;
use std::fs;

#[derive(Copy, Clone)]
struct Section {
    low: usize,
    high: usize,
}

impl Section {
    fn new(low: usize, high: usize) -> Self {
        Self { low, high }
    }

    fn contains(&self, other: Section) -> bool {
        other.low >= self.low && other.high <= self.high
    }
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let file_content =
        fs::read_to_string(&args[0]).expect("Something went wrong reading the file!");

    let lines = file_content.lines();

    let mut count_fully_contained = 0;

    for line in lines {
        let xs: Vec<_> = line
            .split([',', '-'])
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let first_section = Section::new(xs[0], xs[1]);
        let second_section = Section::new(xs[2], xs[3]);

        if first_section.contains(second_section) || second_section.contains(first_section) {
            count_fully_contained += 1;
        }
    }

    println!("Result of puzzle 1: {count_fully_contained}");

    Ok(())
}
