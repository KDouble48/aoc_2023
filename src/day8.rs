use std::collections::HashMap;

struct Map {
    instructions: Vec<char>,
    maps: HashMap<String, [String; 2]>,
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}					

#[aoc_generator(day8)]
fn generator_part1(input: &str) -> Map {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect();
    let mut maps = HashMap::new();
    lines.next();

    while let Some(line) = lines.next() {
        let line = line.replace(&[' ', '(', ')'], "");
        let data: Vec<&str> = line.split("=").collect();
        let name = data[0].to_string();
        let map: Vec<&str> = data[1].split(",").collect();

        maps.insert(name, [map[0].to_string(), map[1].to_string()]);
    }

    Map { instructions, maps }
}

#[aoc(day8, part1)]
fn solve_part1(input: &Map) -> usize {
    let mut current = "AAA".to_string();
    let mut counter = 0;

    while current != "ZZZ" {
        let next;
        match input.instructions[counter % input.instructions.len()] {
            'L' => next = input.maps.get(&current).unwrap()[0].to_owned(),
            'R' => next = input.maps.get(&current).unwrap()[1].to_owned(),
            _ => unreachable!(),
        }

        current = next;
        counter += 1;
    }

    counter
}

#[aoc(day8, part2, Lcm)]
fn solve_part2_lcm(input: &Map) -> usize {
    let currents: Vec<&String> = input
        .maps
        .keys()
        .filter_map(|name| {
            if name.ends_with("A") {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    let mut steps: Vec<usize> = vec![];

    for start in currents {
        let mut current = start.to_owned();
        let mut counter = 0;

        while !current.ends_with("Z") {
            match input.instructions[counter % input.instructions.len()] {
                'L' => current = input.maps.get(&current).unwrap()[0].to_owned(),
                'R' => current = input.maps.get(&current).unwrap()[1].to_owned(),
                _ => unreachable!(),
            }
            counter += 1;
        }

        steps.push(counter);
    }

    let mut lcm_num = steps.pop().unwrap();

    while let Some(next) = steps.pop() {
        lcm_num = lcm(lcm_num, next);
    }

    lcm_num
}

#[aoc(day8, part2, Bad)]
#[allow(unreachable_code, unused_variables)]
// Too slow...
fn solve_part2(input: &Map) -> usize {
    return 0;
    let mut currents: Vec<&String> = input
        .maps
        .keys()
        .filter_map(|name| {
            if name.ends_with("A") {
                Some(name)
            } else {
                None
            }
        })
        .collect();
    let mut counter = 0;

    while !currents.iter().all(|name| name.ends_with("Z")) {

        for i in 0..currents.len() {
            match input.instructions[counter % input.instructions.len()] {
                'L' => currents[i] = &input.maps.get(currents[i]).unwrap()[0],
                'R' => currents[i] = &input.maps.get(currents[i]).unwrap()[1],
                _ => unreachable!(),
            }

        }

        counter += 1;
    }

    counter
}
