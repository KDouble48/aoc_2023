#[aoc(day1, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let mut numbers = vec![];
        for c in line.chars() {
            if c.is_numeric() {
                numbers.push(c);
            }
        }
        if !numbers.is_empty() {
            let num = format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
            result += num.parse::<u32>().unwrap();
        }
    }

    result
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut only_numbers = "".to_string();
    for line in input.lines() {
        let mut r_line = line.to_string();

        for (i, n) in numbers.iter().enumerate() {
            let c = char::from_digit((i + 1) as u32, 10).unwrap();

            while let Some(i) = r_line.find(n) {
                r_line.insert(i+1, c)
            }
        }

        r_line.find("one").and_then(|i| {
            r_line.insert(i + 1, '1');

            Some(())
        });

        only_numbers = format!("{}\n{}", only_numbers, r_line);
    }

    solve_part1(&only_numbers)
}
