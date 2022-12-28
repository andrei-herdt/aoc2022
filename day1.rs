use std::fs;

fn main() {
    let file_path = "input1.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let _v: Vec<&str> = contents
        .split_terminator("\n\n")
        .collect();

    // Try map here
    for calories in _v {
        let split_calories = calories.split_terminator("\n");
        for calory in split_calories {
            let parsed: u32 = calory.parse().unwrap();
        }
    }
    // let _max = _v
    //     .iter_into().split("\n").parse::<u32>().reduce(|acc, x| acc + x);
    // println!("{}", _max);
}
