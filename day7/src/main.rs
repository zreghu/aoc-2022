use std::env;
use std::fs;
use std::collections::HashSet;
use nom::{
    Finish, IResult,
    bytes::complete::{ tag, take_while1 },
    combinator::{ map, all_consuming },
    sequence::{ preceded, separated_pair },
    branch::alt
};

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
enum Entry {
    Dir(String),
    File(u64, String)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Node {
    name: String,
    size: u64,
    children: Vec<Node>,
}

fn parse_path(input: &str) -> IResult<&str, &str> {
    take_while1(|c:char| "abcdefghijklmnopqrstuvwxyz./".contains(c))(input)
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
     map(preceded(tag("cd "), parse_path), |p| Command::Cd(p.to_string()))(input)
}

fn parse_ls(input: &str) -> IResult<&str,Command> {
    map(tag("ls"), |_| Command::Ls)(input)
}

fn parse_dir(input: &str) -> IResult<&str, Entry> {
    map(preceded(tag("dir "), parse_path), |s| Entry::Dir(s.to_string()))(input)
}

fn parse_file(input: &str) -> IResult<&str, Entry> {
    map(separated_pair(nom::character::complete::u64, tag(" "), parse_path), |(i,p)| Entry::File(i,p.to_string()))(input)
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((parse_file, parse_dir))(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(input)?;
    alt((parse_ls, parse_cd))(i)
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
            map(parse_entry, Line::Entry),
            map(parse_command, Line::Command)
    ))(input)
}

fn main() {
    let file_name = env::args().nth(1).expect("there should be a file name specified");

    let contents = fs::read_to_string(file_name).expect("could not read {file_name}");

    let lines = contents
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut root = Node { name: "/".to_string(), children: Vec::new(), size: 0 };

    for line in lines {
        println!("{:?}", line);
        match line {
            Line::Command(Command::Ls) => continue,
            Line::Command(Command::Cd(path)) => continue,
            Line::Entry(Entry::Dir(dir)) => continue,
            Line::Entry(Entry::File(size, name)) => root,
        }
    }

    root.size = 30;
    root.children.push(Node { name: "a.txt".to_string(), children: Vec::new(), size: 29 });
    root.children[0].size = 30;
    println!("{:?}", root);
}
