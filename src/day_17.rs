use std::{fs::File, io::Read, collections::HashMap};

use ndarray::Array2;

fn parse_content(content: &str) -> Array2<i32> {
    let lines = content.lines().collect::<Vec<&str>>();
    let mut array = Array2::zeros((lines.len(), lines.get(0).unwrap().len()));

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            array[(i, j)] = c.to_digit(10).unwrap() as i32;
        }
    }
    array
}

pub fn solve() {
    let mut file = File::open("inputs/day_17_example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let matrix = parse_content(&contents);
    println!("{:?}", matrix);

    let mut unvisited: Vec<(usize, usize)> = Vec::new();
    for i in 0..matrix.shape()[0] {
        for j in 0..matrix.shape()[1] {
            unvisited.push((i, j));
        }
    }

    let distance: HashMap<(usize, usize), i32> = HashMap::new();
    
}