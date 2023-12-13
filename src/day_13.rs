use std::{fs::File, io::Read};

use ndarray::{Array2, Array, s, Axis};

fn parse_to_array(block: &str) -> Array2<i32> {
    let lines = block.split('\n').collect::<Vec<&str>>();
    let mut array = Array2::zeros((lines.len(), lines[0].len()));
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                array[(i, j)] = 1;
            }
        }
    }
    array
}

fn mirror(array: &Array2<i32>) -> i32 {
    for c in 1..array.shape()[1] {
        // we consider the minimum set of columns
        let window = c.min(array.shape()[1] - c);
        let left = array.slice(s![.., (c - window)..c]);
        let mut right = array.slice(s![.., c..(window + c)]);
        right.invert_axis(Axis(1));
        
        if left == right {
            println!("Found mirror at {c}");
            println!("Left \n{:?}", left);
            println!("Right \n{:?}\n\n", right);
            return c as i32;
        }
    }
    0
}

fn mirror_smudge(array: &Array2<i32>) -> i32 {
    for c in 1..array.shape()[1] {
        // we consider the minimum set of columns
        let window = c.min(array.shape()[1] - c);
        let left = array.slice(s![.., (c - window)..c]);
        let mut right = array.slice(s![.., c..(window + c)]);
        right.invert_axis(Axis(1));
        
        if (left.to_owned() - right.to_owned()).map(|x| x.abs()).sum() == 1 {
            println!("Found mirror at {c}");
            println!("Left \n{:?}", left);
            println!("Right \n{:?}\n\n", right);
            return c as i32;
        }
    }
    0
}

pub fn solve() {
    let mut file = File::open("inputs/day_13.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut result = 0;
    for block in contents.split("\n\n").collect::<Vec<&str>>() {
        let array = parse_to_array(block);
        let vertical = mirror(&array);
        //println!("Array {:?}", array);
        let array = array.reversed_axes();
        //println!("Array {:?}", array);
        let horizontal = mirror(&array) * 100;
        result += vertical + horizontal;
    }
    println!("The result is {result}");
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_13.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut result = 0;
    for block in contents.split("\n\n").collect::<Vec<&str>>() {
        let array = parse_to_array(block);
        let vertical = mirror_smudge(&array);
        //println!("Array {:?}", array);
        let array = array.reversed_axes();
        //println!("Array {:?}", array);
        let horizontal = mirror_smudge(&array) * 100;
        result += vertical + horizontal;
    }
    println!("The result is {result}");
}