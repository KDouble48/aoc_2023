#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    mapper: Vec<Mapper>,
}

#[derive(Debug)]
struct Mapper {
    from: Vec<u32>,
    to: Vec<u32>,
    amount: Vec<u32>,
}

impl Mapper {
    fn translate(&self, n: u32) -> u32 {
        for (index, from) in self.from.iter().enumerate() {
            let to = self.to[index];
            let amount = self.amount[index];

            if n >= *from && n < (from + amount as u32) {
                let diff: i32 = to as i32 - *from as i32;

                let result = (n as i32 + diff) as u32;

                return result;
            }
        }
        n
    }
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Almanac {
    let mut almanac = Almanac {
        seeds: vec![],
        mapper: vec![],
    };

    let mut lines = input.lines();

    let seed_line = lines.next().unwrap();
    let seed_numbers: Vec<u32> = seed_line.split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    almanac.seeds = seed_numbers;

    lines.next();

    while let Some(_) = lines.next() {
        // name = name.split(" ").collect::<Vec<&str>>()[0];
        let mut all_numbers: Vec<Vec<u32>> = vec![];
        while let Some(numbers) = lines.next() {
            if numbers.is_empty() {
                break;
            }

            let numbers: Vec<u32> = numbers
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            all_numbers.push(numbers);
        }

        let mut mapper = Mapper {
            from: vec![],
            to: vec![],
            amount: vec![],
        };

        for numbers in all_numbers {
            mapper.from.push(numbers[1]);
            mapper.to.push(numbers[0]);
            mapper.amount.push(numbers[2]);
        }

        almanac.mapper.push(mapper);
    }

    almanac
}

#[aoc(day5, part1)]
fn solve_part1(input: &Almanac) -> u32 {
    let results = input
        .seeds
        .iter()
        .map(|s| {
            let mut current_number = *s;

            for path in &input.mapper {
                current_number = path.translate(current_number);
            }

            current_number
        })
        .collect::<Vec<u32>>();

    *results.iter().min().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &Almanac) -> u32 {
    let mut seeds = vec![];

    for i in (0..input.seeds.len()).step_by(2) {
        let start = input.seeds[i];
        let amount = input.seeds[i + 1];

        for a in 0..amount {
            seeds.push(start + a)
        }
    }

    let results = seeds
        .iter()
        .map(|s| {
            let mut current_number = *s;

            for path in &input.mapper {
                current_number = path.translate(current_number);
            }

            current_number
        })
        .collect::<Vec<u32>>();

    *results.iter().min().unwrap()
}
