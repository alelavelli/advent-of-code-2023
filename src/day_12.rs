use itertools::Itertools;
use std::{fs::File, io::Read, collections::HashSet};

#[derive(Debug, PartialEq)]
enum Cell {
    UNKOWN,
    SEPARATOR,
    BROKEN,
}


fn is_valid(sequence: &Vec<(Cell, usize, usize)>, counts: &Vec<usize>) -> bool {

    let broken_groups = sequence
        .iter()
        .filter(|(cell, start, length)| *cell == Cell::BROKEN)
        .map(|(cell, start, length)| *length)
        .collect::<Vec<usize>>();
    counts == &broken_groups
}

pub fn solve() {
    let mut file = File::open("inputs/day_12.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut valid_combinations = 0;
    for line in lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let first = *split.get(0).unwrap();
        let second = *split.get(1).unwrap();

        // id of the first ? and length
        let mut unknown_groups: Vec<(usize, usize)> = Vec::new();
        let mut separator_groups: Vec<(usize, usize)> = Vec::new();
        let mut broken_groups: Vec<(usize, usize)> = Vec::new();
        // type, index of first id, length
        let mut sequence: Vec<(Cell, usize, usize)> = Vec::new();
        //println!("{:?}", first.split('.').enumerate().filter(|(_, x)| x.len() > 0).collect::<Vec<(usize, &str)>>());
        let first_split: Vec<String> = first
            .chars()
            .group_by(|&x| x)
            .into_iter()
            .map(|(_, r)| r.collect())
            .collect();
        //println!("{:?}", first_split);
        let mut index = 0;
        for group in first_split.iter() {

            let cell = if group.contains('?') {
                unknown_groups.push((index, group.len()));
                Cell::UNKOWN
            } else if group.contains('.') {
                // if it is the first separator group or the last we don't care of it
                if (index == 0) | (index + group.len() == first.len()) {
                    index += group.len();
                    continue;
                }
                separator_groups.push((index, group.len()));
                Cell::SEPARATOR
            } else {
                broken_groups.push((index, group.len()));
                Cell::BROKEN
            };
            sequence.push((cell, index, group.len()));
            index += group.len();
        }
        //println!("Sequence is \n {:?}", sequence);
        //println!("Unkown groups are \n{:?}\nBroken groups are \n{:?}\nSeparator gruops are \n{:?}", unknown_groups, broken_groups, separator_groups);
        let counts = second.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        // now we count the number of broken groups and check if we need to add separators
        let n_broken_groups = second.split(',').collect::<Vec<&str>>().len();
        let n_separators_to_add = n_broken_groups - 1 - separator_groups.len();
        //println!("We need to add {n_separators_to_add} separators to have {n_broken_groups} broken groups");
        // we count the number of broken elements we need to add
        let n_broken_elems: usize = second.split(',').map(|x| x.parse::<usize>().unwrap()).sum();
        let n_broken_to_add: usize = n_broken_elems - broken_groups.iter().map(|(_, x)| x).sum::<usize>();
        //println!("We need to add {n_broken_to_add} broken elems to have {n_broken_elems} broken elems");
        //println!("{:?}", unknown_groups);
        let free_slots = unknown_groups.iter().map(|(_, length)| length).sum::<usize>();
        /*println!(
            "We need to add {} elems and we have {} free slots",
            n_broken_to_add + n_separators_to_add,
            free_slots
        );*/
        if free_slots == n_separators_to_add + n_broken_to_add {
            valid_combinations += 1;
            continue;
        }
        //let combinations = (0..free_slots).permutations(n_broken_to_add + n_separators_to_add).collect::<Vec<Vec<usize>>>();
        let combinations: Vec<Vec<usize>> = (0..free_slots).combinations(n_broken_to_add).collect::<Vec<Vec<usize>>>();
        //println!("Combinations are {:?}", combinations);
        let mut internal_valid_combinations = 0;
        let mut seen_strings: HashSet<String> = HashSet::new();
        for comb in combinations {
            let mut sequence: Vec<(Cell, usize, usize)> = Vec::new();
            let mut index = 0;
            let mut unknown_index: usize = 0;
            let mut new_string = String::new();
            for c in first.chars() {
                new_string.push(match c {
                    '?' => {
                        unknown_index += 1;
                        if comb.contains(&(unknown_index - 1)) {
                        //if comb.get(unknown_index - 1).unwrap() < &n_broken_to_add {
                            '#'
                        } else {
                            '.'
                        }
                    }
                    _ => c
                }
            );
            }
            if seen_strings.contains(&new_string) {
                continue;
            }
             else {
                seen_strings.insert(new_string.clone());
             }
            //println!("new string {:?}", new_string);
            let string_split: Vec<String> = new_string
            .chars()
            .group_by(|&x| x)
            .into_iter()
            .map(|(_, r)| r.collect())
            .collect();
            for group in string_split.iter() {

                let cell = if group.contains('?') {
                    unknown_index += 1;
                    if comb.get(unknown_index - 1).unwrap() < &n_broken_to_add {
                        Cell::BROKEN
                    } else {
                        Cell::SEPARATOR
                    }
                } else if group.contains('.') {
                    // if it is the first separator group or the last we don't care of it
                    if (index == 0) | (index + group.len() == first.len()) {
                        index += group.len();
                        continue;
                    }
                    Cell::SEPARATOR
                } else {
                    Cell::BROKEN
                };
                sequence.push((cell, index, group.len()));
                index += group.len();
            }

            if is_valid(&sequence, &counts) {
                valid_combinations += 1;
                internal_valid_combinations += 1;
            }
        }
        println!("internal valid combinations are {internal_valid_combinations}");
    }
    println!("Valid combinations are {valid_combinations}");

}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_12_example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut valid_combinations = 0;
    for line in lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let first = *split.get(0).unwrap();
        let second = *split.get(1).unwrap();

        let mut unfolded_first = String::new();
        let mut unfolded_second = String::new();

        for i in 0..5 {
            if i >= 1 {
                unfolded_first.push('?');
                unfolded_second.push(',');
            }
            unfolded_first.push_str(first);
            unfolded_second.push_str(second);
        }
        let first = unfolded_first.as_str();
        let second = unfolded_second.as_str();

        // id of the first ? and length
        let mut unknown_groups: Vec<(usize, usize)> = Vec::new();
        let mut separator_groups: Vec<(usize, usize)> = Vec::new();
        let mut broken_groups: Vec<(usize, usize)> = Vec::new();
        // type, index of first id, length
        let mut sequence: Vec<(Cell, usize, usize)> = Vec::new();
        //println!("{:?}", first.split('.').enumerate().filter(|(_, x)| x.len() > 0).collect::<Vec<(usize, &str)>>());
        let first_split: Vec<String> = first
            .chars()
            .group_by(|&x| x)
            .into_iter()
            .map(|(_, r)| r.collect())
            .collect();
        //println!("{:?}", first_split);
        let mut index = 0;
        for group in first_split.iter() {

            let cell = if group.contains('?') {
                unknown_groups.push((index, group.len()));
                Cell::UNKOWN
            } else if group.contains('.') {
                // if it is the first separator group or the last we don't care of it
                if (index == 0) | (index + group.len() == first.len()) {
                    index += group.len();
                    continue;
                }
                separator_groups.push((index, group.len()));
                Cell::SEPARATOR
            } else {
                broken_groups.push((index, group.len()));
                Cell::BROKEN
            };
            sequence.push((cell, index, group.len()));
            index += group.len();
        }
        //println!("Sequence is \n {:?}", sequence);
        //println!("Unkown groups are \n{:?}\nBroken groups are \n{:?}\nSeparator gruops are \n{:?}", unknown_groups, broken_groups, separator_groups);
        let counts = second.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        // now we count the number of broken groups and check if we need to add separators
        let n_broken_groups = second.split(',').collect::<Vec<&str>>().len();
        let n_separators_to_add = n_broken_groups - 1 - separator_groups.len();
        //println!("We need to add {n_separators_to_add} separators to have {n_broken_groups} broken groups");
        // we count the number of broken elements we need to add
        let n_broken_elems: usize = second.split(',').map(|x| x.parse::<usize>().unwrap()).sum();
        let n_broken_to_add: usize = n_broken_elems - broken_groups.iter().map(|(_, x)| x).sum::<usize>();
        //println!("We need to add {n_broken_to_add} broken elems to have {n_broken_elems} broken elems");
        //println!("{:?}", unknown_groups);
        let free_slots = unknown_groups.iter().map(|(_, length)| length).sum::<usize>();
        /*println!(
            "We need to add {} elems and we have {} free slots",
            n_broken_to_add + n_separators_to_add,
            free_slots
        );*/
        if free_slots == n_separators_to_add + n_broken_to_add {
            valid_combinations += 1;
            continue;
        }
        //let combinations = (0..free_slots).permutations(n_broken_to_add + n_separators_to_add).collect::<Vec<Vec<usize>>>();
        let combinations: Vec<Vec<usize>> = (0..free_slots).combinations(n_broken_to_add).collect::<Vec<Vec<usize>>>();
        //println!("Combinations are {:?}", combinations);
        let mut internal_valid_combinations = 0;
        let mut seen_strings: HashSet<String> = HashSet::new();
        for comb in combinations {
            let mut sequence: Vec<(Cell, usize, usize)> = Vec::new();
            let mut index = 0;
            let mut unknown_index: usize = 0;
            let mut new_string = String::new();
            for c in first.chars() {
                new_string.push(match c {
                    '?' => {
                        unknown_index += 1;
                        if comb.contains(&(unknown_index - 1)) {
                        //if comb.get(unknown_index - 1).unwrap() < &n_broken_to_add {
                            '#'
                        } else {
                            '.'
                        }
                    }
                    _ => c
                }
            );
            }
            if seen_strings.contains(&new_string) {
                continue;
            }
             else {
                seen_strings.insert(new_string.clone());
             }
            //println!("new string {:?}", new_string);
            let string_split: Vec<String> = new_string
            .chars()
            .group_by(|&x| x)
            .into_iter()
            .map(|(_, r)| r.collect())
            .collect();
            for group in string_split.iter() {

                let cell = if group.contains('?') {
                    unknown_index += 1;
                    if comb.get(unknown_index - 1).unwrap() < &n_broken_to_add {
                        Cell::BROKEN
                    } else {
                        Cell::SEPARATOR
                    }
                } else if group.contains('.') {
                    // if it is the first separator group or the last we don't care of it
                    if (index == 0) | (index + group.len() == first.len()) {
                        index += group.len();
                        continue;
                    }
                    Cell::SEPARATOR
                } else {
                    Cell::BROKEN
                };
                sequence.push((cell, index, group.len()));
                index += group.len();
            }

            if is_valid(&sequence, &counts) {
                valid_combinations += 1;
                internal_valid_combinations += 1;
            }
        }
        println!("internal valid combinations are {internal_valid_combinations}");
    }
    println!("Valid combinations are {valid_combinations}");

}