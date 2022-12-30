use std::fs;
use std::time::{ Instant};

fn main() {
    let file_path = "input1.txt";

    let start = Instant::now();
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let elves: Vec<&str> = input
        .split_terminator("\n\n")
        .collect();

    let mut total_calories: Vec<u32> = elves.iter().map(
        |word| word.split_terminator("\n")
        .map(
            |cal| cal.parse::<u32>()
            .unwrap()
            ).collect::<Vec<u32>>().iter().sum()
        ).collect::<Vec<u32>>();

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);


    println!("{}", total_calories.iter().max().unwrap());

    let start= Instant::now();
    total_calories.sort();
    total_calories.reverse();

    let biggest_three = &total_calories[0..3];
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    println!("{}", biggest_three.iter().sum::<u32>());
}
