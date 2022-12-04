use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::ops::Index;


const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;
const WIN_VALUE: i32 = 6;
const DRAW_VALUE: i32 = 3;

fn main() {
    let raw_strategy_guide = match fill_vec_from_file("input.txt") {
        Ok(content) => content,
        Err(e) => panic!("Panic: {}", e),
    };

    let strategy_guide = parse_strategy_guide_from_text(raw_strategy_guide);

    println!("Strategy guide: {:?}", strategy_guide);

    let mut acc: i32 = 0;

    for (a, b) in strategy_guide.iter() {
        let result = shapes_to_points(*a, *b);
        println!("Result: {}", result);
        acc += result

    }

    println!("Final score: {}", acc)

}

fn parse_strategy_guide_from_text(lines: Vec<String>) -> Vec<(i32, i32)> {
    let strategy_vector = lines.iter().map(|line|
        line.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<_>>();
    let characters_strategy_guide: Vec<(&str, &str)> = strategy_vector.iter().map(|pair| (*pair.index(0), *pair.index(1))).collect();
    let strategy_guide = characters_strategy_guide.iter().map(|tuple| match tuple {
        ("A", "X") => (ROCK, ROCK),
        ("A", "Y") => (ROCK, PAPER),
        ("A", "Z") => (ROCK, SCISSORS),
        ("B", "X") => (PAPER, ROCK),
        ("B", "Y") => (PAPER, PAPER),
        ("B", "Z") => (PAPER, SCISSORS),
        ("C", "X") => (SCISSORS, ROCK),
        ("C", "Y") => (SCISSORS, PAPER),
        ("C", "Z") => (SCISSORS, SCISSORS),
        _ => (0, 0)
    }).collect::<Vec<_>>();
    strategy_guide
}

fn fill_vec_from_file(filename: &str) -> Result<Vec<String>, Error> {
    let file = match File::open(filename) {
        Ok(returned_file) => returned_file,
        Err(e) => {
            return Err(e)
        }
    };
    let reader = BufReader::new(file);

    let mut vector: Vec<String> = vec![];

    for (index, line) in reader.lines().enumerate() {
        let line_content = match line {
            Ok(content) => content,
            Err(e) => return Err(e)
        };

        vector.insert(index, line_content);
    }

    Ok(vector)

}

fn shapes_to_points(shape_a: i32, shape_b: i32) -> i32 {
    // Player A chooses Paper
    return if shape_a == PAPER {
        // Player B wins
        if shape_b == SCISSORS { SCISSORS + WIN_VALUE }
        // Player B loses
        else if shape_b == ROCK { ROCK }
        // Draw
        else { PAPER + DRAW_VALUE }
    } else if shape_a == ROCK {
        if shape_b == PAPER { PAPER + WIN_VALUE } else if shape_b == SCISSORS { SCISSORS } else { ROCK + DRAW_VALUE }
    } else {
        if shape_b == ROCK { ROCK + WIN_VALUE } else if shape_b == PAPER { PAPER } else { SCISSORS + DRAW_VALUE }
    }
}

#[test]
fn find_total_score(){
    let strategy_guide: Vec<(i32, i32)> = vec![(ROCK, PAPER), (PAPER, ROCK), (SCISSORS, SCISSORS)];

    println!("Strategy guide: {:?}", strategy_guide);

    let mut acc: i32 = 0;

    for (a, b) in strategy_guide.iter() {
        let result = shapes_to_points(*a, *b);
        println!("Result: {}", result);
        acc += result

    }

    println!("Final score: {}", acc)
}