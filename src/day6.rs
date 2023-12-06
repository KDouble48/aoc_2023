struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn hold_button(&self, duration: u64) -> u64 {
        if duration >= self.time {
            return 0
        }

        duration * (self.time - duration)
    }

    fn possible_wins(&self) -> Vec<u64> {
        let mut wins = vec![];
        
        for i in 1..self.time {
            let distance = self.hold_button(i);

            if distance > self.distance {
                wins.push(i);
            }
        }

        wins
    }
}

#[aoc_generator(day6, part1)]
fn generator_part1(input: &str) -> Vec<Race> {
    let mut races = vec![];

    let mut input_iter = input.lines();

    let times: Vec<u64> = input_iter.next().unwrap().split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let distances: Vec<u64> = input_iter.next().unwrap().split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        })
    }

    races
}

#[aoc_generator(day6, part2)]
fn generator_part2(input: &str) -> Race {
    let input = input.replace(" ", "");

    let mut input_iter = input.lines();

    let time = input_iter.next().unwrap().split(":").collect::<Vec<&str>>()[1].parse().unwrap();
    let distance = input_iter.next().unwrap().split(":").collect::<Vec<&str>>()[1].parse().unwrap();

    Race { time, distance }
}

#[aoc(day6, part1)]
fn solve_part1(input: &[Race]) -> usize {
    let mut possible_winning_times: Vec<usize> = input.iter().map(|r| r.possible_wins().len()).collect();

    let mut result = possible_winning_times.pop().unwrap();

    for t in possible_winning_times {
        result *= t;
    }

    result
}

#[aoc(day6, part2)]
fn solve_part2(input: &Race) -> usize {
    input.possible_wins().len()
}
