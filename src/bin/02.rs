use std::env;
use std::fs;

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
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

    let mut total_score = 0;

    for line in lines {
        if let Some((first, second)) = line.split_once(' ') {
            let opponent_shape = match first {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => unreachable!(),
            };

            let my_shape = match second {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => unreachable!(),
            };

            total_score += calculate_score(opponent_shape, my_shape);
        }
    }

    println!("Result of puzzle 1: {total_score}");

    Ok(())
}

fn calculate_score(opponent_shape: Shape, my_shape: Shape) -> usize {
    let game_score = match (opponent_shape, my_shape) {
        // opponent wins
        (Shape::Rock, Shape::Scissors)
        | (Shape::Scissors, Shape::Paper)
        | (Shape::Paper, Shape::Rock) => 0,
        // draw
        (Shape::Scissors, Shape::Scissors)
        | (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper) => 3,
        // I win
        _ => 6,
    };

    my_shape.score() + game_score
}
