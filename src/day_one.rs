use std::fs;

fn main() {

    let filename = "input/day_one_input.txt";

    let input = fs::read_to_string(filename)
        .expect("Failed to load  input\n");

    let input = input.trim().split('\n');

    let mut total = 0;
    let mut total_history = vec![ total ];

    for elem in input {
        let num: i32 = elem.parse().expect("Couldn't parse");
        total = total + num;
        if val_seen_before(&total_history, total) {
            println!("Seen before: {}", total);
            break;
        };
        total_history.push(total);
    }

    println!("Total: {}", total);
}

fn val_seen_before(total_history: &Vec<i32>, current_val: i32) -> bool {
   let seen_before = if total_history.contains(&current_val) {
        true
    } else {
        false
    };

   println!("Seen before: {} for {}", seen_before, current_val);
   seen_before
}
