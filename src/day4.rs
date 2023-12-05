#[derive(Clone)]
struct ScratchCard {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    amount: u32,
}

impl ScratchCard {
    fn matches(&self) -> u32 {
        let mut matches = 0;
        for winning_number in &self.winning_numbers {
            for number in &self.numbers {
                if winning_number == number {
                    matches += 1;
                }
            }
        }
        matches
    }

    fn points(&self) -> u32 {
        let matches = self.matches();

        if matches != 0 {
            return 2_u32.pow(matches - 1);
        }
        0
    }
}

#[aoc_generator(day4)]
fn generator(input: &str) -> Vec<ScratchCard> {
    let mut scratch_cards = vec![];

    for mut line in input.lines() {
        line = line.split(":").collect::<Vec<&str>>()[1];
        let all_numbers = line.split("|").collect::<Vec<&str>>();
        let winning_numbers: Vec<u32> = all_numbers[0]
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        let numbers: Vec<u32> = all_numbers[1]
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let scratch_card = ScratchCard {
            winning_numbers,
            numbers,
            amount: 1,
        };

        scratch_cards.push(scratch_card);
    }

    scratch_cards
}

#[aoc(day4, part1)]
fn solve_part1(input: &[ScratchCard]) -> u32 {
    let mut results = vec![];

    for card in input {
        results.push(card.points())
    }
    results.iter().sum()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[ScratchCard]) -> u32 {
    let mut input = input.to_vec();

    for i in 0..input.len() {
        let matches = input[i].matches();

        for j in (i + 1)..=(i + matches as usize) {
            input[j].amount += input[i].amount;
        }
    }

    input
        .iter()
        .map(|i| i.amount)
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}
