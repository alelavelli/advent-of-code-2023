use std::{fs::File, io::Read, collections::HashMap};

fn hash_fn(input: &str) -> usize {
    let mut current_value = 0;
    for c in input.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value = current_value % 256;
    }
    current_value
}

pub fn solve() {
    let mut file = File::open("inputs/day_15.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let ascii_codes: Vec<usize> = contents.replace('\n', "").split(',').map(hash_fn).collect();
    println!("Result is {}", ascii_codes.iter().sum::<usize>());
}


#[derive(Debug)]
struct LensesBox<'a > {
    // k = label, v = focal length
    lenses: HashMap<&'a str, usize>,
    // k = label
    order: HashMap<&'a str, usize>,
}

fn split_step(step: &str) -> (&str, char, usize) {
    if step.contains('=') {
        let split = step.split('=').collect::<Vec<&str>>();
        (
            *split.get(0).unwrap(),
            '=',
            split.get(1).unwrap().parse::<usize>().unwrap()
        )
    } else {
        let split = step.split('-').collect::<Vec<&str>>();
        (
            *split.get(0).unwrap(),
            '-',
            0
        )
    }
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_15.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents = contents.replace('\n', "");
    
    let mut boxes: HashMap<usize, LensesBox> = HashMap::new();
    let contents_iter = contents.split(',');
    for step in contents_iter {
        let (label, operation, focal_length) = split_step(step);
        let box_code: usize = hash_fn(label);

        if operation == '-' {
            boxes.entry(box_code).and_modify(
                |box_entry| {
                    box_entry.lenses.remove(label);
                    if let Some(lense_position) = box_entry.order.remove(label) {
                        //box_entry.order.remove(label);
                        for x in box_entry.order.values_mut() {
                            if *x > lense_position {
                                *x -= 1;
                            }
                        }
                    }
                }
            );
        } else if operation == '=' {
            boxes.entry(box_code).and_modify(
                |box_entry| {
                    box_entry.lenses
                        .entry(label)
                        .and_modify(|lense| *lense = focal_length)
                        .or_insert(focal_length);
                    if !box_entry.order.contains_key(label) {
                        // add the new label at first position and add by 1 all the other values
                        /*for x in box_entry.order.values_mut() {
                            *x += 1;
                        }*/
                        box_entry.order.insert(label, box_entry.lenses.len() - 1);
                    }
                }
            ).or_insert(
                {
                    let mut lenses = HashMap::new();
                    lenses.insert(label, focal_length);
                    let mut order = HashMap::new();
                    order.insert(label, 0);
                    LensesBox { 
                        lenses ,
                        order
                    }
                }
            );
        } else {
            panic!("unknown operation");
        }
        //println!("{:#?}", boxes);
    }

    let mut power = 0;
    for (box_number, lenses_box) in boxes.iter() {
        for label in lenses_box.lenses.keys() {
            power+= (box_number + 1) 
                * (lenses_box.order.get(label).unwrap() + 1)
                * (lenses_box.lenses.get(label).unwrap());
        }
    }
    println!("Power is {power}");
}