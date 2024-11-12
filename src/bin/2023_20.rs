#![allow(unused)]
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/2023/20.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let mut modules: Vec<Box<dyn Module>> = vec![];

    input.lines().for_each(|line| {
        let name = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .get(1..)
            .unwrap()
            .to_string();
        if line.starts_with('%') {
            modules.push(Box::new(FlipFlop::new(name)));
        } else if line.starts_with('&') {
            modules.push(Box::new(Conjunction::new(name)));
        }
    });

    0
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Voltage {
    Low,
    High,
}

trait Module<'a> {
    fn recv(&mut self, src: String, voltage: Voltage);
    fn set_connections(&mut self, connections: Vec<&'a mut Box<dyn Module<'a>>>);
    fn say_name(&self);
    fn name(&self) -> String;
}

struct Broadcaster<'a> {
    connections: Vec<&'a mut Box<dyn Module<'a>>>,
}

struct FlipFlop<'a> {
    name: String,
    is_on: bool,
    connections: Vec<&'a mut Box<dyn Module<'a>>>,
}

struct Conjunction<'a> {
    name: String,
    memory: HashMap<String, Voltage>,
    connections: Vec<&'a mut Box<dyn Module<'a>>>,
}

impl<'a> Module<'a> for Broadcaster<'a> {
    fn recv(&mut self, _src: String, voltage: Voltage) {
        for conn in &mut self.connections {
            conn.recv("broadcaster".to_string(), voltage);
        }
    }

    fn set_connections(&mut self, _connections: Vec<&'a mut Box<dyn Module>>) {}

    fn say_name(&self) {
        println!("broadcaster");
    }

    fn name(&self) -> String {
        String::from("Broadcaster")
    }
}

impl<'a> Conjunction<'a> {
    fn new(name: String) -> Self {
        Self {
            name,
            memory: HashMap::new(),
            connections: vec![],
        }
    }
}

impl<'a> FlipFlop<'a> {
    fn new(name: String) -> Self {
        Self {
            name,
            is_on: false,
            connections: vec![],
        }
    }
}

impl<'a> Module<'a> for FlipFlop<'a> {
    fn recv(&mut self, _src: String, voltage: Voltage) {
        if voltage == Voltage::Low {
            if self.is_on {
                for conn in &mut self.connections {
                    conn.recv(self.name.clone(), Voltage::High);
                }
            }
            self.is_on = !self.is_on;
        }
    }

    fn set_connections(&mut self, connections: Vec<&'a mut Box<dyn Module<'a>>>) {
        self.connections = connections;
    }

    fn say_name(&self) {
        println!("flipflop {}", self.name);
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl<'a> Module<'a> for Conjunction<'a> {
    fn recv(&mut self, src: String, voltage: Voltage) {
        self.memory.insert(src, voltage);
        if self.memory.values().all(|v| v == &Voltage::High) {
            self.connections.iter_mut().for_each(|conn| {
                conn.recv(self.name.clone(), Voltage::Low);
            });
        }
    }

    fn set_connections(&mut self, connections: Vec<&'a mut Box<dyn Module<'a>>>) {
        self.connections = connections;
    }

    fn say_name(&self) {
        println!("Conjunction: {}", self.name);
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(
            r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        );
        assert_eq!(actual, 32_000_000);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
