use std::env;
use std::fs;

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
enum Plan {
    Win,
    Draw,
    Loss
}

fn main() {
    let file_path: String = env::args().nth(1).unwrap();

    let contents = fs::read_to_string(file_path)
        .expect("while reading {file_path}");

    let score = contents
        .lines()
        .map(parse_line)
        .map(calculate_turn)
        .sum::<u64>();
    
    println!("Score using guide is {score}");
}

fn parse_line(line: &str) -> (RPS, Plan) {
    let turn = line
        .split(' ')
        .collect::<Vec<_>>();

    let opponent = match turn[0] {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _   => panic!("Invalid turn {}", turn[0])
    };

    let plan = match turn[1] {
        "X" => Plan::Loss,
        "Y" => Plan::Draw,
        "Z" => Plan::Win,
        _   => panic!("Invalid turn {}", turn[1])
    };
           
    return (opponent, plan);
}

fn calculate_turn(turn : (RPS, Plan)) -> u64 {
    let result_points = match turn {
        (RPS::Rock, Plan::Loss)     => 0 + 3,
        (RPS::Paper, Plan::Loss)    => 0 + 1,
        (RPS::Scissors, Plan::Loss) => 0 + 2,
        (RPS::Rock, Plan::Draw)     => 3 + 1,
        (RPS::Paper, Plan::Draw)    => 3 + 2,
        (RPS::Scissors, Plan::Draw) => 3 + 3,
        (RPS::Rock, Plan::Win)      => 6 + 2,
        (RPS::Paper, Plan::Win)     => 6 + 3,
        (RPS::Scissors, Plan::Win)  => 6 + 1
    };

    return result_points
}
