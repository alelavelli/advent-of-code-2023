use std::{collections::HashMap, fs::File, io::Read};

pub fn solve() {
    let mut file = File::open("inputs/day_02_example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut available_cubes: HashMap<&str, u32> = HashMap::new();
    available_cubes.insert("red", 12);
    available_cubes.insert("green", 13);
    available_cubes.insert("blue", 14);

    let mut possible_games = 0;
    for (i, game) in lines.iter().enumerate() {
        // remove game header and then get th throws
        let throws = game.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .collect::<Vec<&str>>();
        // loop for each throw and if we found something that is not possible then we go outside the loop
        let mut possible = true;
        for throw in throws {
            for cubes in throw.split(",").collect::<Vec<&str>>() {
                if cubes.contains("blue") {
                    let num = cubes
                        .replace(" ", "")
                        .trim_end_matches("blue")
                        .parse::<u32>()
                        .unwrap();
                    if num > *available_cubes.get("blue").unwrap() {
                        possible = false;
                        break;
                    }
                } else if cubes.contains("green") {
                    let num = cubes
                        .replace(" ", "")
                        .trim_end_matches("green")
                        .parse::<u32>()
                        .unwrap();
                    if num > *available_cubes.get("green").unwrap() {
                        possible = false;
                        break;
                    }
                } else if cubes.contains("red") {
                    let num = cubes
                        .replace(" ", "")
                        .trim_end_matches("red")
                        .parse::<u32>()
                        .unwrap();
                    if num > *available_cubes.get("red").unwrap() {
                        possible = false;
                        break;
                    }
                } else {
                    println!("mmm");
                }
            }
        }
        if possible {
            possible_games += i + 1;
        }
    }
    println!("Sum of possible games is: {possible_games}");
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_02_pt2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut games_power: u32 = 0;

    for game in lines.iter() {
        let mut max_blue = 0;
        let mut max_green = 0;
        let mut max_red = 0;

        // remove game header and then get th throws
        let throws = game.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .collect::<Vec<&str>>();

        for throw in throws {
            for cubes in throw.split(",").collect::<Vec<&str>>() {
                if cubes.contains("blue") {
                    let num = cubes
                        .replace(" ", "")
                        .trim_end_matches("blue")
                        .parse::<u32>()
                        .unwrap();
                    if num > max_blue {
                        max_blue = num;
                    }
                } else if cubes.contains("green") {
                    let num = cubes
                        .replace(" ", "")
                        .trim_end_matches("green")
                        .parse::<u32>()
                        .unwrap();
                    if num > max_green {
                        max_green = num;
                    }
                } else if cubes.contains("red") {
                    let num = cubes
                        .replace(" ", "")
                        .trim_end_matches("red")
                        .parse::<u32>()
                        .unwrap();
                    if num > max_red {
                        max_red = num;
                    }
                } else {
                    println!("mmm");
                }
            }
        }
        games_power += max_blue * max_green * max_red
    }
    println!("Sum of games' power is: {games_power}");
}
