#[derive(Debug)]
pub struct GameData {
    id: usize,
    pulls: Vec<Pull>,
}

#[derive(Debug)]
pub struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

#[aoc_generator(day2)]
fn generator(input: &str) -> Vec<GameData> {
    let mut gamedatas = vec![];
    
    for line in input.lines() {
        let mut gamedata = GameData {
            id: 0,
            pulls: vec![],
        };
        let s1 = line.split(":").collect::<Vec<&str>>();
        gamedata.id = s1[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        let s2 = s1[1].split(";").collect::<Vec<&str>>();

        let colors = ["red", "green", "blue"];

        for data in s2 {
            let mut pull = Pull {
                red: 0,
                green: 0,
                blue: 0,
            };
            for c_data in data.split(",") {
                let s3 = c_data.split(" ").collect::<Vec<&str>>();
                for color in colors {
                    if let Some(_) = s3[2].find(color) {
                        if color == "red" {
                            pull.red = s3[1].parse().unwrap();
                        }
                        if color == "green" {
                            pull.green = s3[1].parse().unwrap();
                        }
                        if color == "blue" {
                            pull.blue = s3[1].parse().unwrap();
                        }
                    }
                }
            }
            gamedata.pulls.push(pull);
        }
        gamedatas.push(gamedata);
    }
    gamedatas
}

#[aoc(day2, part1)]
fn solve_part1(input: &[GameData]) -> usize {
    let mut ids = vec![];
    for game in input {
        ids.push(game.id);
        for pull in &game.pulls {
            if pull.red > 12 || pull.green > 13 || pull.blue > 14 {
                ids.pop();
                break;
            }
        }
    }
    ids.iter().sum()
}

#[aoc(day2, part2)]
fn solve_part2(input: &[GameData]) -> u32 {
    let mut powers = vec![];
    for game in input {
        let mut fewest = Pull {red: 0, green: 0, blue: 0};
        for pull in &game.pulls {
            if pull.red > fewest.red {
                fewest.red = pull.red;
            }
            if pull.green > fewest.green {
                fewest.green = pull.green;
            }
            if pull.blue > fewest.blue {
                fewest.blue = pull.blue;
            }
        }
        powers.push(fewest.red * fewest.green * fewest.blue);
    }
    powers.iter().sum()
}
