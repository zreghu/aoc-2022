use std::env;
use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let file_path: String = env::args().nth(1).unwrap();

    let contents = fs::read_to_string(file_path)
        .expect("while reading {file_path}");

    let groups = contents
        .lines()
        .map(|line| line.bytes().collect::<HashSet<u8>>())
        .collect::<Vec<_>>();

    let priority: u64 = groups.iter()
        .tuples()
        .map(|(a, b, c)| a.iter()
             .filter(|x| b.contains(x))
             .filter(|x| c.contains(x))
             .next().copied().unwrap())
        .map(get_priority)
        .sum();

    println!("Total priority is {priority}");
}

fn get_priority(c: u8) -> u64 {
    let p = match c {
        b'a'..=b'z' => (c - b'a' + 1).into(),
        b'A'..=b'Z' => (c - b'A' + 27).into(),
        _         => 0
    };
    return dbg!(p);
}
