use std::{fs::File, io::Read};

fn solve_diseq(a: i128, b: i128, c: i128) -> (i128, i128) {
    let sqrt = ((b * b - 4 * a * c) as f32).sqrt();
    (
        ((-b as f32 + sqrt) / 2.0 * a as f32).ceil() as i128,
        ((-b as f32 - sqrt) / 2.0 * a as f32).floor() as i128,
    )
    
}

pub fn solve() {
    let mut file = File::open("inputs/day_06.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut times: Vec<i128> = Vec::new();
    let mut distances: Vec<i128> = Vec::new();

    for c in lines.get(0).unwrap().split(":").collect::<Vec<&str>>()[1].trim().split_whitespace() {
        times.push(c.parse::<i128>().unwrap());
    }

    for c in lines.get(1).unwrap().split(":").collect::<Vec<&str>>()[1].trim().split_whitespace() {
        distances.push(c.parse::<i128>().unwrap());
    }
    let mut result = 1;
    for i in 0..times.len() {
        let (a, b) = solve_diseq(-1, *times.get(i).unwrap(), -(distances.get(i).unwrap() + 1));
        result *= b - a + 1;
        println!("Solution for {i} is {a}-{b}", );
    }
    println!("times {:?}", times);
    println!("distances {:?}", distances);
    println!("Result is {result}");
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_06.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();


    let time  = lines.get(0).unwrap().split(":").collect::<Vec<&str>>()[1].trim().replace(" ", "").parse::<i128>().unwrap();
    let distance = lines.get(1).unwrap().split(":").collect::<Vec<&str>>()[1].trim().replace(" ", "").parse::<i128>().unwrap();
    
    let (a, b) = solve_diseq(-1, time, -distance + 1);
    let result = b - a;
    println!("Solution is {a}-{b}", );
    println!("time {:?}", time);
    println!("distance {:?}", distance);
    println!("Result is {result}");
}