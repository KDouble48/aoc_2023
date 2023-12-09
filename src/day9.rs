#[aoc(day9, part1)]
fn solve_part1(input: &str) -> i32 {
    let mut results = vec![];

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut numbers_vec = vec![numbers];
        while !numbers_vec.last().unwrap().iter().all(|n| *n == 0) {
            let mut new_numbers = vec![];

            let mut numbers_iter = numbers_vec.last().unwrap().iter();
            let mut previous = numbers_iter.next().unwrap();

            while let Some(current) = numbers_iter.next() {
                let diff = current - previous;
                new_numbers.push(diff);
                previous = current;
            }

            numbers_vec.push(new_numbers);
        }

        for i in (0..numbers_vec.len()).rev() {
            if i == numbers_vec.len()-1 {
                numbers_vec[i].push(0);
                continue;
            }

            let new_value = numbers_vec[i].last().unwrap() + numbers_vec[i+1].last().unwrap();
            numbers_vec[i].push(new_value);
        }

        results.push(*numbers_vec[0].last().unwrap());
    }

    results.iter().sum()
}

#[aoc(day9, part2)]
fn solve_part2(input: &str) -> i32 {
    let mut results = vec![];

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut numbers_vec = vec![numbers];
        while !numbers_vec.last().unwrap().iter().all(|n| *n == 0) {
            let mut new_numbers = vec![];

            let mut numbers_iter = numbers_vec.last().unwrap().iter();
            let mut previous = numbers_iter.next().unwrap();

            while let Some(current) = numbers_iter.next() {
                let diff = previous - current;
                new_numbers.push(diff);
                previous = current;
            }

            numbers_vec.push(new_numbers);
        }

        for i in (0..numbers_vec.len()).rev() {
            if i == numbers_vec.len()-1 {
                numbers_vec[i].push(0);
                continue;
            }

            let new_value = numbers_vec[i].first().unwrap() + numbers_vec[i+1].first().unwrap();
            numbers_vec[i].insert(0,new_value);
        }

        results.push(*numbers_vec[0].first().unwrap());
    }

    results.iter().sum()
}
