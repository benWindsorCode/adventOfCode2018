use std::fs;
use std::env;

fn main() {

    let filename = "input/day_one_input.txt";

    let input = fs::read_to_string(filename)
        .expect("Failed to load  input\n");

    let input = input.trim().split('\n');

    let mut total = 0;

    for elem in input {
        let num: i32 = elem.parse().expect("Couldn't parse");
        total = total + num;
    }

    println!("Total: {}", total);


}
