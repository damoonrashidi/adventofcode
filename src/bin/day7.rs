use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();

    println!("{}", puzzle_one(&file));
    println!("{}", puzzle_two(&file));
}

#[derive(Debug)]
enum Op {
    CD(String),
    LS(),
}

fn parse<'a>(line: &'a str) -> Op {
    if line.contains("cd") {
        return Op::CD(line.split(" ").last().unwrap().to_owned());
    }
    Op::LS()
}

fn puzzle_one(input: &String) -> u32 {
    let dirs = HashMap::<String, u32>::new();
    let mut cd = "/".to_string();

    for line in input.lines() {
        match parse(line) {
            Op::CD(dir) => {
                cd = dir;

                dirs.insert(dir, v)
            }
            Op::LS() => todo!(),
        }
    }

    0
}

fn puzzle_two(_input: &String) -> u32 {
    0
}
