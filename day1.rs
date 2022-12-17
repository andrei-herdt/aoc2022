use std::fs;

fn main() {
    let file_path = "input1.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for line in contents.chars(){
        println!("{}", line.to_digit(10).to_string());
    }

    println!("With text:\n{contents}");
}
