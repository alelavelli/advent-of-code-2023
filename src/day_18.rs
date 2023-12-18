use std::{fs::File, io::Read};

use geo::{Polygon, LineString, Area, Contains, point};
use indicatif::{ProgressBar, ProgressIterator};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

impl Direction {
    fn from_char(input: &char) -> Direction {
        match input {
            'R' => Direction::RIGHT,
            'D' => Direction::DOWN,
            'U' => Direction::UP,
            'L' => Direction::LEFT,
            _ => panic!()
        }
    }
}

fn parse_directives(contents: &str) -> Vec<(Direction, i64)> {
    let mut directives = Vec::new();
    for line in contents.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        directives.push((
            Direction::from_char(&split.get(0).unwrap().chars().nth(0).unwrap()),
            split.get(1).unwrap().parse::<i64>().unwrap()
        ))
    }
    directives
}

fn print_matrix(coordinates: &Vec<(f32, f32)>) {
    for i in 0..1+coordinates.iter().map(|(a, b)| (*a as i64, *b as i64)).max_by_key(|x| x.0).map(|x| x.0).unwrap() {
        for j in 0..1+coordinates.iter().map(|(a, b)| (*a as i64, *b as i64)).max_by_key(|x| x.1).map(|x| x.1).unwrap() {
            if coordinates.contains(&(i as f32, j as f32)) {
                print!("#");
            } else{
                print!(".");
            }
        }
        println!()
    }
    println!();
    println!();
}

pub fn solve() {
    let mut file = File::open("inputs/day_18_example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let directives = parse_directives(&contents);

    let mut coordinates_vec: Vec<(f32, f32)> = Vec::new();
    let mut current_point: (f32, f32) = (0.0, 0.0);
    coordinates_vec.push(current_point.clone());
    for directive in directives {
        match directive.0 {
            Direction::LEFT => {
                for _ in 0..directive.1 {
                    current_point.1 -= 1.0;
                    coordinates_vec.push(current_point.clone());
                }
            },
            Direction::RIGHT => {
                for _ in 0..directive.1 {
                    current_point.1 += 1.0;
                    coordinates_vec.push(current_point.clone());
                }
            },
            Direction::DOWN => {
                for _ in 0..directive.1 {
                    current_point.0 += 1.0;
                    coordinates_vec.push(current_point.clone());
                }
            },
            Direction::UP => {
                for _ in 0..directive.1 {
                    current_point.0 -= 1.0;
                    coordinates_vec.push(current_point.clone());
                }
          }
        };
        //print_matrix(&coordinates_vec);
    }
    let polygon = Polygon::new(
        LineString::from(coordinates_vec.clone()),
        vec![],
    );
    println!("Area is {}", polygon.unsigned_area());    

    let mut cells = coordinates_vec.len() - 1;
    let min_row = coordinates_vec.iter().map(|(a, b)| (*a as i64, *b as i64)).min_by_key(|x| x.0).map(|x| x.0).unwrap();
    let max_row = coordinates_vec.iter().map(|(a, b)| (*a as i64, *b as i64)).max_by_key(|x| x.0).map(|x| x.0).unwrap();
    let min_col = coordinates_vec.iter().map(|(a, b)| (*a as i64, *b as i64)).min_by_key(|x| x.1).map(|x| x.1).unwrap();
    let max_col = coordinates_vec.iter().map(|(a, b)| (*a as i64, *b as i64)).max_by_key(|x| x.1).map(|x| x.1).unwrap();
    for i in min_row..(1 + max_row) {
        for j in min_col..(1 + max_col) {
            if polygon.contains(&point!(x: i as f32, y: j as f32)) {
                cells += 1;
            }
        }
    }
    println!("There are {} cells", cells);
    //print_matrix(&coordinates_vec);
}


fn parse_directives_pt2(contents: &str) -> Vec<(Direction, i64)> {
    let mut directives = Vec::new();
    for line in contents.lines() {
        let split = String::from(*line.split_whitespace().collect::<Vec<&str>>().last().unwrap()).replace("(", "").replace(")", "");

        let direction = match split.chars().last().unwrap() {
            '0' => Direction::RIGHT,
            '1' => Direction::DOWN,
            '2' => Direction::LEFT,
            '3' => Direction::UP,
            _ => panic!("ops")
        };

        let mut chars = split.chars();
        chars.next();
        chars.next_back();
        let step = i64::from_str_radix(&chars.as_str(), 16).unwrap();
        directives.push((
            direction,
            step
        ))
    }
    directives
}


pub fn solve_pt2() {
    let mut file = File::open("inputs/day_18_example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let directives = parse_directives_pt2(&contents);
    let directives = parse_directives(&contents);

    let mut coordinates_vec: Vec<(f32, f32)> = Vec::new();
    let mut current_point: (f32, f32) = (0.0, 0.0);
    coordinates_vec.push(current_point.clone());
    for directive in directives {
        match directive.0 {
            Direction::LEFT => {
                for _ in 0..directive.1 {
                    current_point.1 -= 1.0;
                    coordinates_vec.push(current_point.clone());
                }
            },
            Direction::RIGHT => {
                for _ in 0..directive.1 {
                    current_point.1 += 1.0;
                    coordinates_vec.push(current_point.clone());
                }
            },
            Direction::DOWN => {
                for _ in 0..directive.1 {
                    current_point.0 += 1.0;
                    coordinates_vec.push(current_point.clone());
                }
            },
            Direction::UP => {
                for _ in 0..directive.1 {
                    current_point.0 -= 1.0;
                    coordinates_vec.push(current_point.clone());
                }
          }
        };
        //print_matrix(&coordinates_vec);
    }
    let polygon = Polygon::new(
        LineString::from(coordinates_vec.clone()),
        vec![],
    );
    
    let mut cells = coordinates_vec.len() - 1;
    let min_row = coordinates_vec.iter().map(|(a, b)| (*a as i64, *b as i64)).min_by_key(|x| x.0).map(|x| x.0).unwrap();
    let max_row = coordinates_vec.iter().map(|(a, b)| (*a as i64, *b as i64)).max_by_key(|x| x.0).map(|x| x.0).unwrap();
    let min_col = coordinates_vec.iter().map(|(a, b)| (*a as i64, *b as i64)).min_by_key(|x| x.1).map(|x| x.1).unwrap();
    let max_col = coordinates_vec.iter().map(|(a, b)| (*a as i64, *b as i64)).max_by_key(|x| x.1).map(|x| x.1).unwrap();
    for i in (min_row..(1 + max_row)).progress_with(ProgressBar::new((max_row - min_row) as u64)) {
        for j in min_col..(1 + max_col) {
            if polygon.contains(&point!(x: i as f32, y: j as f32)) {
                cells += 1;
            }
        }
    }
    println!("There are {} cells", cells);
}
