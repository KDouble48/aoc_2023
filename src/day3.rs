fn check_bewtween(start: i32, len: i32, to_check: Vec<&str>) -> bool {
    let end = (start + len).min((to_check[0].len() - 1) as i32);
    let start = (start - 1).max(0);

    for a in to_check {
        let chars: Vec<char> = a.chars().collect();
        for idx in start..=end {
            let c = chars[idx as usize];
            if !c.is_numeric() && c != '.' {
                return true;
            }
        }
    }

    false
}

fn get_gear_ratio(at: i32, to_check: Vec<&str>) -> u32 {
    let end = (at + 1).min((to_check[0].len() - 1) as i32);
    let start = (at - 1).max(0);
    let mut found_numbers = vec![];

    let mut got_number = false;

    for (i, a) in to_check.iter().enumerate() {
        let chars: Vec<char> = a.chars().collect();
        for c_i in start..=end {
            let char = chars[c_i as usize];
            if char.is_numeric() {
                if got_number {
                    continue;
                }
                found_numbers.push(get_number(c_i.clone() as usize, to_check[i]));
                got_number = true;
            } else {
                got_number = false;
            }
        }
        got_number = false;
    }
    
    if found_numbers.len() == 2 {
        return found_numbers[0] * found_numbers[1];
    }
    0
}

fn get_number(mut at: usize, s: &str) -> u32 {
    let s_vec: Vec<char> = s.chars().collect();
    let mut number = "".to_string();

    while at > 0 && s_vec[at - 1].is_numeric() {
        at -= 1;
    }

    while at < s_vec.len() && s_vec[at].is_numeric() {
        number.push(s_vec[at]);
        at += 1;
    }

    number.parse().unwrap()
}

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut numbers: Vec<u32> = vec![];
    let lines: Vec<&str> = input.lines().collect();

    for (idx, line) in lines.iter().enumerate() {
        let mut current_number = "".to_string();
        for (char_idx, char) in line.chars().enumerate() {
            if char.is_numeric() {
                current_number.push(char);
            }
            if (!char.is_numeric() || char_idx == lines[0].len() - 1) && current_number.len() != 0 {
                let lines: Vec<&str> = input.lines().collect();

                let mut lines_to_check = vec![lines[idx]];
                if idx != 0 {
                    lines_to_check.push(lines[idx - 1]);
                }
                if idx != lines.len() - 1 {
                    lines_to_check.push(lines[idx + 1]);
                }

                if check_bewtween(
                    (char_idx - current_number.len()) as i32,
                    current_number.len() as i32,
                    lines_to_check,
                ) {
                    numbers.push(current_number.parse().unwrap());
                }
                current_number.clear();
            }
        }
    }

    numbers.iter().sum()
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut numbers: Vec<u32> = vec![];
    let lines: Vec<&str> = input.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        for (c_i, char) in line.chars().enumerate() {
            if char == '*' {
                let mut lines_to_check: Vec<_> = vec![lines[i]];
                if i != 0 {
                    lines_to_check.push(lines[i - 1]);
                }
                if i != lines.len() - 1 {
                    lines_to_check.push(lines[i + 1]);
                }
                numbers.push(get_gear_ratio(c_i as i32, lines_to_check));
            }
        }
    }

    numbers.iter().sum()
}
