use std::collections::HashSet;
use std::env;
use std::fs;

type Position = (isize, isize, isize);

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let file_content =
        fs::read_to_string(&args[0]).expect("Something went wrong reading the file!");

    const CUBE_SURFACES: usize = 6;

    let positions = parse(&file_content);
    let mut connected_surfaces = 0;

    for (x, y, z) in &positions {
        let (x, y, z) = (*x, *y, *z);
        let neighbors = vec![
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ];

        for neighbor in neighbors {
            if positions.contains(&neighbor) {
                connected_surfaces += 1;
            }
        }
    }

    let result = (positions.len() * CUBE_SURFACES) - connected_surfaces;

    println!("Result of puzzle 1: {result}");

    Ok(())
}

fn parse(s: &str) -> HashSet<Position> {
    let mut positions = HashSet::new();

    for line in s.lines() {
        let position: Vec<isize> = line.split(',').map(|e| e.parse().unwrap()).collect();
        positions.insert((position[0], position[1], position[2]));
    }

    positions
}
