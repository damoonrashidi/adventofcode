fn main() {
    let input = include_str!("../../inputs/2015/14.txt").trim();

    println!("puzzle one: {}", puzzle_one(input, 2503));
    println!("puzzle two: {}", puzzle_two(input, 2503));
}

#[derive(Debug)]
#[allow(unused)]
struct Raindeer {
    speed: usize,
    flight_duration: usize,
    rest_duration: usize,
}

fn parse(input: &str) -> Vec<Raindeer> {
    let mut deers = vec![];
    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<_>>();
        let speed = words[3].parse::<usize>().unwrap();
        let flight_duration = words[6].parse::<usize>().unwrap();
        let rest = words[13].parse::<usize>().unwrap();

        let deer = Raindeer {
            speed,
            flight_duration,
            rest_duration: rest,
        };

        deers.push(deer);
    }

    deers
}

#[allow(clippy::cast_possible_wrap)]
fn puzzle_one(input: &str, limit: usize) -> usize {
    let deers = parse(input);
    let mut best = 0;

    for deer in deers {
        let flights = limit / (deer.flight_duration + deer.rest_duration);
        let remainder = limit % (deer.flight_duration + deer.rest_duration);
        let distance = flights * deer.speed * deer.flight_duration
            + deer.speed * remainder.min(deer.flight_duration);
        if distance > best {
            best = distance;
        }
    }

    best
}

fn puzzle_two(input: &str, limit: usize) -> usize {
    input.len() + limit
}

#[cfg(test)]
mod tests {

    #[test]
    fn comet() {
        assert_eq!(
            crate::puzzle_one(
                r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
                1000,
            ),
            1120
        );
    }
    #[test]
    fn dancer() {
        assert_eq!(
            crate::puzzle_one(
                r"Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
                1000,
            ),
            1056
        );
    }
}
