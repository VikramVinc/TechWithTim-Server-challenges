use std::io;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_cases = input.trim().parse::<i32>().unwrap();

    for i in 0..num_cases {
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let letters = input.trim().chars().collect::<HashSet<char>>();

        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let q = input.trim().parse::<i32>().unwrap();

        let mut total_length = 0;
        for j in 0..q {
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let word = input.trim();

            // Check if all the letters in 'word' are in 'letters'
            if word.chars().all(|c| letters.contains(&c)) {
                total_length += word.len();
            }
        }
        println!("{}", total_length);
    }
}
