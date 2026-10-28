use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let file_name = env::args().nth(1).expect("there should be a file name specified");

    let contents = fs::read_to_string(file_name).expect("could not read {file_name}");

    const WINDOW : usize = 14;

    let prefix_num : usize = contents
        .lines()
        .map(|line| line
                .as_bytes()
                .windows(WINDOW)
                .enumerate()
                .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == WINDOW)
                .map(|(i, _)| dbg!(i + WINDOW))
            )
        .map(|t| match t { Some(x) => x, None => 0 })
        .sum();

    println!("Sum of character prefixes is {prefix_num}");
}
