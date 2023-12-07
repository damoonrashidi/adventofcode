use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/2015/7.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
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

    let mut all_instructions_executed = false;

    while !all_instructions_executed {
        let instructions = input.trim().lines().map(|str| {
            let (instruction_str, dst) = str.split_once(" -> ").unwrap();
            (Instruction::from(instruction_str), dst.to_string())
        });
        instructions.for_each(|(instruction, dst)| {
            let mut has_moved = false;

            match instruction {
                Instruction::Assign(value) => {
                    println!("static assign {value} to {dst}");
                    board.insert(dst, value);
                    has_moved = true;
                }
                Instruction::AssignDynamic(src) => {
                    if let Some(a) = board.get(&src) {
                        println!("dynamic assign {src} ({a}) to {dst}");
                        board.insert(dst, *a);
                        has_moved = true;
                    }
                }
                Instruction::Or(src, target) => {
                    if let Some(a) = board.get(&src) {
                        let b = board.get(&target).unwrap_or(&0);
                        println!("{src} OR {target} {a} | {b} to {dst}");
                        board.insert(dst, a | b);
                        has_moved = true;
                    }
                }
                Instruction::And(src, target) => {
                    if let (Some(a), Some(b)) = (board.get(&src), board.get(&target)) {
                        println!("{src} AND {target} {a} & {b} to {dst}");
                        board.insert(dst, a & b);
                        has_moved = true;
                    }
                }
                Instruction::LShift(src, amount) => {
                    if let Some(a) = board.get(&src) {
                        println!("{src} << {amount} {a} to {dst}");
                        board.insert(dst, a << amount);
                        has_moved = true;
                    }
                }
                Instruction::RShift(src, amount) => {
                    if let Some(a) = board.get(&src) {
                        println!("{src} >> {amount} {a} to {dst}");
                        board.insert(dst, a >> amount);
                        has_moved = true;
                    }
                }
                Instruction::Not(src) => {
                    if let Some(a) = board.get(&src) {
                        println!("NOT {src} {a} to {dst}");
                        board.insert(dst, !a);
                        has_moved = true;
                    }
                }
            }
            all_instructions_executed = has_moved && board.keys().any(|src| src.as_str() == "a");
        });
    }

    dbg!(board);

    0
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
