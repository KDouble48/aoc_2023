use std::collections::HashMap;

fn card_to_value(c: char) -> u8 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn card_to_value_part2(c: char) -> u8 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn cards_to_value(cards: Vec<u8>) -> u8 {
    if cards.is_empty() {
        return 0;
    }

    let mut cards_iter = cards.iter();
    let current = cards_iter.next().unwrap();
    let mut counter = 1;
    let mut remaining: Vec<u8> = vec![];

    while let Some(next) = cards_iter.next() {
        if next != current {
            remaining = cards_iter.map(|i| *i).collect();
            remaining.insert(0, *next);
            break;
        }
        counter += 1;
    }

    // High card: 5
    // One pair: 6
    // Two pair: 7
    // Three of a kind: 8
    // Full house: 9
    // Four of a kind: 11
    // Five of a kind: 12

    match counter {
        1 => 1 + cards_to_value(remaining),
        2 => 3 + cards_to_value(remaining),
        3 => 6 + cards_to_value(remaining),
        4 => 10 + cards_to_value(remaining),
        5 => 12,
        _ => unreachable!(),
    }
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
struct Hand {
    hand_value: u8,
    cards: Vec<u8>,
    bid: u32,
}

#[aoc_generator(day7, part1)]
fn generator_part1(input: &str) -> Vec<Hand> {
    let mut hands = vec![];

    for line in input.lines() {
        let binding = line.split(" ").collect::<Vec<&str>>();
        let mut values = binding.iter();

        let cards: Vec<u8> = values
            .next()
            .unwrap()
            .chars()
            .map(|c| card_to_value(c))
            .collect();

        let mut cards_copy = cards.clone();
        cards_copy.sort();

        let bid = values.next().unwrap().parse().unwrap();
        let hand_value = cards_to_value(cards_copy);

        let hand = Hand {
            cards,
            bid,
            hand_value,
        };

        hands.push(hand);
    }

    hands
}

#[aoc_generator(day7, part2)]
fn generator_part2(input: &str) -> Vec<Hand> {
    let mut hands = vec![];

    for line in input.lines() {
        let binding = line.split(" ").collect::<Vec<&str>>();
        let mut values = binding.iter();

        let cards: Vec<u8> = values
            .next()
            .unwrap()
            .chars()
            .map(|c| card_to_value_part2(c))
            .collect();

        let mut cards_copy = cards.clone();
        cards_copy.sort();

        let mut hash_map = HashMap::new();

        for i in &cards_copy {
            if *i == 1 {
                continue;
            }

            hash_map.entry(*i).and_modify(|v| *v += 1).or_insert(1);
        }

        if let Some(most) = hash_map
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
        {
            for i in 0..cards_copy.len() {
                if cards_copy[i] == 1 {
                    cards_copy[i] = *most;
                }
            }
        }
        cards_copy.sort();

        let bid = values.next().unwrap().parse().unwrap();

        let hand_value = cards_to_value(cards_copy);

        let hand = Hand {
            cards,
            bid,
            hand_value,
        };

        hands.push(hand);
    }

    hands
}

#[aoc(day7, part1)]
fn solve_part1(input: &[Hand]) -> u32 {
    let mut hands = input.to_vec();
    hands.sort();

    let mut result = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += hand.bid * (i as u32 + 1);
    }

    result
}

#[aoc(day7, part2)]
fn solve_part2(input: &[Hand]) -> u32 {
    solve_part1(input)
}
