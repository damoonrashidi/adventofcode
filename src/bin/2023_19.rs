use regex::RegexBuilder;
use std::{cmp::Ordering, collections::HashMap};

type PartsList = Vec<(usize, usize, usize, usize)>;

#[derive(Debug)]
struct Condition {
    op: Ordering,
    part: Part,
    amount: usize,
    action: Action,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Part {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Action {
    Approve,
    Reject,
    Workflow(String),
}

#[derive(Debug)]
struct Workflow {
    name: String,
    filters: Vec<Condition>,
    action: Action,
}

fn main() {
    let input = include_str!("../../inputs/2023/19.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

fn puzzle_one(input: &str) -> usize {
    let (parts, workflows) = parse(input);

    let mut accepted: Vec<usize> = vec![];

    let mut wf_name = String::from("in");
    'outer: for (x, m, a, s) in parts {
        'wf_loop: while let Some(wf) = workflows.get(&wf_name) {
            for condition in &wf.filters {
                let match_on = match condition.part {
                    Part::X => x,
                    Part::M => m,
                    Part::A => a,
                    Part::S => s,
                };
                if condition.amount.cmp(&match_on) != condition.op {
                    match &condition.action {
                        Action::Approve => {
                            accepted.push(x + m + a + s);
                            wf_name = String::from("in");
                            continue 'outer;
                        }
                        Action::Reject => {
                            wf_name = String::from("in");
                            continue 'outer;
                        }
                        Action::Workflow(name) => {
                            wf_name = name.clone();
                            continue 'wf_loop;
                        }
                    }
                }
            }
            match &wf.action {
                Action::Approve => {
                    accepted.push(x + m + a + s);
                    wf_name = "in".to_string();
                    continue 'outer;
                }
                Action::Reject => {
                    wf_name = "in".to_string();
                    continue 'outer;
                }
                Action::Workflow(name) => {
                    wf_name = name.to_string();
                }
            }
        }
    }

    accepted.iter().sum()
}

fn puzzle_two(input: &str) -> usize {
    input.len()
}

fn parse(input: &str) -> (PartsList, HashMap<String, Workflow>) {
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();

    let mut workflows = HashMap::new();

    workflows_str.lines().map(parse_workflow).for_each(|w| {
        workflows.insert(w.name.clone(), w);
    });

    let parts = parts_str
        .lines()
        .map(|line| {
            let v: Vec<usize> = line
                .replace(['{', '}'], "")
                .split(',')
                .map(|c| c.split('=').last().unwrap().parse::<usize>().unwrap())
                .collect();

            (v[0], v[1], v[2], v[3])
        })
        .collect();

    (parts, workflows)
}

fn parse_workflow(line: &str) -> Workflow {
    let filters_regex =
        RegexBuilder::new(r"(?P<part>[amxs])(?P<operand><|>)(?P<amount>[\d]+):(?P<action>[a-z]+)")
            .case_insensitive(true)
            .build()
            .unwrap();

    let (name, action) = {
        let (name, _) = line.split_once('{').unwrap();
        let action = match line.split(',').last().unwrap().replace('}', "").as_str() {
            "A" => Action::Approve,
            "R" => Action::Reject,
            workflow => Action::Workflow(workflow.to_string()),
        };
        (name.to_string(), action)
    };

    let mut conditions = vec![];
    for filter in filters_regex.captures_iter(line) {
        let amount = filter.name("amount").unwrap().as_str().parse().unwrap();
        let part = match filter.name("part").unwrap().as_str() {
            "x" => Part::X,
            "m" => Part::M,
            "a" => Part::A,
            "s" => Part::S,
            _ => unreachable!(),
        };
        let op = match filter.name("operand").unwrap().as_str() {
            "<" => Ordering::Less,
            ">" => Ordering::Greater,
            _ => unreachable!(),
        };
        let action = match filter.name("action").unwrap().as_str() {
            "A" => Action::Approve,
            "R" => Action::Reject,
            workflow => Action::Workflow(workflow.to_string()),
        };

        conditions.push(Condition {
            op,
            part,
            amount,
            action,
        });
    }

    Workflow {
        name,
        filters: conditions,
        action,
    }
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(
            r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(actual, 19114);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(r"");
        assert_eq!(actual, 0);
    }
}
