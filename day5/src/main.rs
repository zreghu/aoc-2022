use std::env;
use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct CargoStack {
    stack: Vec<Vec<char>>,
}

impl CargoStack {
    fn init(&mut self, size: usize) -> () {
        for _ in 0..size {
            self.stack.push(Vec::new());
        }
    }

    fn load_stack(&mut self, input: &Vec<Vec<char>>) -> () {
        for (i, val) in input.iter()
            .rev()
            .skip(1)
            .flat_map(|line| line.iter().enumerate()) 
        {
            if !val.is_whitespace() {
                self.stack[i].push(*val)
            }
        }
    }

    fn move_items(&mut self, num: usize, from: usize, to: usize) {
        let mut buffer: Vec<char> = Vec::new();
        for _ in 0..num {
            let i = self.stack[from].pop();
            buffer.push(i.expect("invalid move, not enought items in stack"));
        }
        
        for i in buffer.iter().rev() {
            self.stack[to].push(*i);
        }
    }

    fn get_top(&mut self) -> Vec<char> {
        let mut top = Vec::new();
        for s in &self.stack {
            top.push(s.last().copied().unwrap_or(' '));
        };
        return top;
    }
}

fn main() {
    let file_name = env::args().nth(1).expect("must specify file name in args");

    let contents = fs::read_to_string(file_name)
        .expect("could not read {file_name}");

    let (stack, moves) = contents
        .split("\n\n")
        .collect_tuple()
        .expect("must define stack and moves deliminated by empty line");

    let mut cargo_stack = CargoStack { stack: Vec::new() };

    let stacks = stack
        .lines()
        .map(|line| line.chars()
             .chunks(4)
             .into_iter()
             .map(|c| c.collect::<Vec<_>>()[1])
             .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let move_list = moves
        .lines()
        .map(|line| { 
            let Some((_, a, _, b, _, c)) = line.split(" ")
                .collect_tuple() else { panic!("bad line") };
            (a, b, c)
        })
        .map(|(a,b,c)| (
                a.parse::<usize>().expect("num of items should be usize"),
                b.parse::<usize>().expect("stack from should be usize") - 1,
                c.parse::<usize>().expect("stack to should be usize") - 1
        ))
        .collect::<Vec<_>>();

    cargo_stack.init(dbg!(stacks[0].len()));
    cargo_stack.load_stack(&stacks);
    println!("Inital Stacks:\n{cargo_stack:?}");

    for (num, from, to) in move_list.iter() {
        cargo_stack.move_items(*num, *from, *to);
        println!("({num}, {from}, {to}) -> {cargo_stack:?}");
    }

    println!("Top is {:?}", cargo_stack.get_top().into_iter().collect::<String>());

}
