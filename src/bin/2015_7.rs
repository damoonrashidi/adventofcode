use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/2015/7.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle one: {}", puzzle_two(input));
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Assign(u16),
    AssignDynamic(String),
    Or(String, String),
    And(String, String),
    LShift(String, u16),
    RShift(String, u16),
    Not(String),
}

fn puzzle_one(input: &str) -> u16 {
    let mut board: HashMap<String, u16> = HashMap::new();

    let instructions = input.trim().lines().map(|str| {
        let (instruction_str, dst) = str.split_once(" -> ").unwrap();
        (Instruction::from(instruction_str), dst.to_string())
    });

    instructions.for_each(|(instruction, dst)| match instruction {
        Instruction::Assign(value) => {
            println!("static assign {value} to {dst}");
            board.insert(dst, value);
        }
        Instruction::AssignDynamic(src) => {
            let a = board.get(&src).unwrap_or(&0);
            println!("dynamic assign {src} ({a}) to {dst}");
            board.insert(dst, *a);
        }
        Instruction::Or(src, target) => {
            let a = board.get(&src).unwrap_or(&0);
            let b = board.get(&target).unwrap_or(&0);
            println!("{src} OR {target} {a} | {b} to {dst}");
            board.insert(dst, a | b);
        }
        Instruction::And(src, target) => {
            let a = board.get(&src).unwrap_or(&0);
            let b = board.get(&target).unwrap_or(&0);
            println!("{src} AND {target} {a} & {b} to {dst}");
            board.insert(dst, a & b);
        }
        Instruction::LShift(src, amount) => {
            let a = board.get(&src).unwrap_or(&0);
            println!("{src} << {amount} {a} to {dst}");
            board.insert(dst, a << amount);
        }
        Instruction::RShift(src, amount) => {
            let a = board.get(&src).unwrap_or(&0);
            println!("{src} >> {amount} {a} to {dst}");
            board.insert(dst, a >> amount);
        }
        Instruction::Not(src) => {
            let a = board.get(&src).unwrap_or(&0);
            println!("NOT {src} {a} to {dst}");
            board.insert(dst, !a);
        }
    });

    *board.get("a").unwrap()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if value.contains("NOT") {
            let (_, dst) = value.split_once(' ').unwrap();
            return Instruction::Not(dst.to_string());
        } else if value.contains(" OR ") {
            let (src, dst) = value.split_once(" OR ").unwrap();
            return Instruction::Or(src.to_string(), dst.to_string());
        } else if value.contains(" AND ") {
            let (src, dst) = value.split_once(" AND ").unwrap();
            return Instruction::And(src.to_string(), dst.to_string());
        } else if value.contains(" LSHIFT ") {
            let (src, amount_str) = value.split_once(" LSHIFT ").unwrap();
            let amount = amount_str.parse::<u16>().unwrap();
            return Instruction::LShift(src.to_string(), amount);
        } else if value.contains(" RSHIFT ") {
            let (src, amount_str) = value.split_once(" RSHIFT ").unwrap();
            let amount = amount_str.parse::<u16>().unwrap();
            return Instruction::RShift(src.to_string(), amount);
        }

        if let Ok(amount) = value.trim().parse() {
            Instruction::Assign(amount)
        } else {
            Instruction::AssignDynamic(value.trim().to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle_one;

    #[test]
    fn test() {
        let actual = puzzle_one(
            r"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        );

        assert_eq!(actual, 0);
    }
}
