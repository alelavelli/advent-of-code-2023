use std::{fs::File, io::Read};

use ndarray::{Array1, s, Axis, stack};


pub fn solve() {
    let mut file = File::open("inputs/day_09.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut sequences: Vec<Array1<i128>> = Vec::new();
    for line in lines {
        sequences.push(
            Array1::from_iter(line.split_whitespace().map(|x| x.parse::<i128>().unwrap()))
        );
    }
    //println!("Sequences are {:#?}", sequences);
    let mut last_elements: Vec<i128> = Vec::new();
    for sequence in sequences {
        let mut steps: Vec<Array1<i128>> = Vec::new();
        steps.push(sequence.clone());
        let mut x = sequence;

        while !x.iter().all(|&x| x == 0) {
            //println!("x is {:?}", x.map(|x| x.abs()));
            
            x = &x.slice(s![1..]) - &x.slice(s![..-1]);
            steps.push(x.clone());
        }
        println!("Steps before zeros are {:#?}", steps);
        // rollback and add elements
        
        steps.last_mut().unwrap().append(Axis(0), Array1::from_elem(1, 0).view()).unwrap();
        for i in (0..(steps.len() - 1)).rev() {
            let diff = steps.get(i + 1).unwrap().last().unwrap().clone();
            let prev_num = steps.get(i).unwrap().last().unwrap().clone();
            let elem = diff + prev_num;
            steps.get_mut(i).unwrap().append(
                Axis(0),
                Array1::from_elem(1, elem).view()
            ).unwrap();
            if i == 0 {
                last_elements.push(elem);
            }
        }
        
        println!("Steps are {:#?}", steps);
        println!();
    }
    println!("Last elements is {:?}", last_elements);
    println!("Result is {}", last_elements.iter().sum::<i128>());
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_09.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut sequences: Vec<Array1<i128>> = Vec::new();
    for line in lines {
        sequences.push(
            Array1::from_iter(line.split_whitespace().map(|x| x.parse::<i128>().unwrap()))
        );
    }
    //println!("Sequences are {:#?}", sequences);
    let mut last_elements: Vec<i128> = Vec::new();
    for sequence in sequences {
        let mut steps: Vec<Array1<i128>> = Vec::new();
        steps.push(sequence.slice(s![..;-1]).to_owned());
        let mut x = sequence;

        while !x.iter().all(|&x| x == 0) {
            //println!("x is {:?}", x.map(|x| x.abs()));
            
            x = &x.slice(s![1..]) - &x.slice(s![..-1]);
            steps.push(x.slice(s![..;-1]).to_owned());
        }
        println!("Steps before zeros are {:#?}", steps);
        // rollback and add elements
        
        steps.last_mut().unwrap().append(Axis(0), Array1::from_elem(1, 0).view()).unwrap();
        for i in (0..(steps.len() - 1)).rev() {
            let diff = steps.get(i + 1).unwrap().last().unwrap().clone();
            let prev_num = steps.get(i).unwrap().last().unwrap().clone();
            let elem = - diff + prev_num;
            steps.get_mut(i).unwrap().append(
                Axis(0),
                Array1::from_elem(1, elem).view()
            ).unwrap();
            if i == 0 {
                last_elements.push(elem);
            }
        }
        
        println!("Steps are {:#?}", steps);
        println!();
    }
    println!("Last elements is {:?}", last_elements);
    println!("Result is {}", last_elements.iter().sum::<i128>());
}