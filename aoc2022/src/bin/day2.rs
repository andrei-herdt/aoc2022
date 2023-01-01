use std::fs;
use std::time::{Duration};
use cpu_time::ProcessTime;


fn evaluate(patt: &str) -> u32{ 
    match patt {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,

        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,

        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
            _ => 0
    }
}

/// # Example
/// ```
/// let result = aoc2022::evaluate("A Y");
/// assert_eq!(result, 8);
/// ```
fn replace(patt: &str) -> &str{ 
    match patt {
        "A X" => "A Z",
        "A Y" => "A X",
        "A Z" => "A Y",

        "B X" => "B X",
        "B Y" => "B Y",
        "B Z" => "B Z",

        "C X" => "C Y",
        "C Y" => "C Z",
        "C Z" => "C X",
            _ => ""
    }
}

fn main() {
    let file_path = "input2.txt";

    let start = ProcessTime::now();
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let input2: Vec<&str>= input.split("\n").collect();

    let sum = input2
        .iter()
        .fold(0, |acc, w| {
            acc+evaluate(w)
        }
                );

    let sum1 = input2
        .iter()
        .map(|w| replace(w))
        .fold(0, |acc, w| {
            acc+evaluate(w)
        }
                );

    let duration: Duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    println!("first result: {}", sum);
    println!("second result: {}", sum1);

}
