use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../inputs/2015/9.txt").trim();

    println!("puzzle one: {}", puzzle_one(input));
    println!("puzzle two: {}", puzzle_two(input));
}

#[derive(Debug, Clone)]
struct State<'a, 'b> {
    pub current_path: Vec<&'a str>,
    pub visited: HashSet<&'a str>,
    pub current_cost: usize,
    pub best: Option<(Vec<&'a str>, usize)>,
    cities: &'b HashSet<&'b str>,
}

impl<'a, 'b> State<'a, 'b> {
    fn is_complete(&self) -> bool {
        self.visited.len() == self.cities.len()
    }
}

fn parse(input: &str) -> (HashMap<&str, HashMap<&str, usize>>, HashSet<&str>) {
    let mut map: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    let mut cities = HashSet::new();

    for line in input.lines() {
        let (places, distance) = line.split_once(" = ").unwrap();
        let distance = distance.parse::<usize>().unwrap();
        let (from, to) = places.split_once(" to ").unwrap();
        if let Some(destinations) = map.get_mut(from) {
            destinations.insert(to, distance);
        } else {
            let mut destinations = HashMap::new();
            destinations.insert(to, distance);
            map.insert(from, destinations);
        }
        if let Some(destinations) = map.get_mut(to) {
            destinations.insert(from, distance);
        } else {
            let mut destinations = HashMap::new();
            destinations.insert(from, distance);
            map.insert(to, destinations);
        }

        cities.insert(from);
        cities.insert(to);
    }

    (map, cities)
}

fn find_shortest<'a>(
    state: &mut State<'a, 'a>,
    map: &'a HashMap<&'a str, HashMap<&'a str, usize>>,
) {
    if state.is_complete() {
        if let Some((_, best_cost)) = state.best {
            if state.current_cost < best_cost {
                state.best = Some((state.current_path.clone(), state.current_cost));
            }
        } else {
            state.best = Some((state.current_path.clone(), state.current_cost));
        }
        return;
    }

    let city = state.current_path.last().unwrap();
    let possible_destinations = map.get(city).unwrap();

    for (next, cost) in possible_destinations {
        if state.visited.contains(next) {
            continue;
        }

        state.visited.insert(next);
        state.current_path.push(next);
        state.current_cost += cost;

        find_shortest(state, map);

        state.visited.remove(next);
        state.current_path.pop();
        state.current_cost -= cost;
    }
}

fn find_longest<'a>(state: &mut State<'a, 'a>, map: &'a HashMap<&'a str, HashMap<&'a str, usize>>) {
    if state.is_complete() {
        if let Some((_, best_cost)) = state.best {
            if state.current_cost > best_cost {
                state.best = Some((state.current_path.clone(), state.current_cost));
            }
        } else {
            state.best = Some((state.current_path.clone(), state.current_cost));
        }
        return;
    }

    let city = state.current_path.last().unwrap();
    let possible_destinations = map.get(city).unwrap();

    for (next, cost) in possible_destinations {
        if state.visited.contains(next) {
            continue;
        }

        state.visited.insert(next);
        state.current_path.push(next);
        state.current_cost += cost;

        find_longest(state, map);

        state.visited.remove(next);
        state.current_path.pop();
        state.current_cost -= cost;
    }
}

fn puzzle_one(input: &str) -> usize {
    let (map, cities) = parse(input);
    let mut lowest = usize::MAX;

    for city in &cities {
        let mut state = State {
            cities: &cities,
            current_path: vec![city],
            visited: HashSet::from_iter([*city]),
            current_cost: 0,
            best: None,
        };

        find_shortest(&mut state, &map);

        println!("{:?}", state.best);

        if let Some((_, best)) = state.best {
            if best < lowest {
                lowest = best;
            }
        }
    }

    lowest
}

fn puzzle_two(input: &str) -> usize {
    let (map, cities) = parse(input);
    let mut highest = 0;

    for city in &cities {
        let mut state = State {
            cities: &cities,
            current_path: vec![city],
            visited: HashSet::from_iter([*city]),
            current_cost: 0,
            best: None,
        };

        find_longest(&mut state, &map);

        println!("{:?}", state.best);

        if let Some((_, best)) = state.best {
            if best > highest {
                highest = best;
            }
        }
    }

    highest
}

#[cfg(test)]
mod tests {
    use crate::{puzzle_one, puzzle_two};

    #[test]
    fn test_puzzle_one() {
        let actual = puzzle_one(
            r"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        );
        assert_eq!(actual, 605);
    }

    #[test]
    fn test_puzzle_two() {
        let actual = puzzle_two(
            r"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        );
        assert_eq!(actual, 982);
    }
}
