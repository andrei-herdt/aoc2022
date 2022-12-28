use std::fs;

fn main() {
    let file_path = "input1.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let _v = contents
        .split("\n\n")
        .inspect(|x| println!("{x}"))
        .collect::<String>();

    // Try map here
    let _max = _v.parse::<u32>().into_iter().reduce(|acc, x| acc + x).unwrap();
    println!("{}", _max);
}
