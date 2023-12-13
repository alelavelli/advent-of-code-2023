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
        let n_separators_to_add = (n_broken_groups as i32 - 1 - separator_groups.len() as i32).max(0) as usize;
        //println!("We need to add {n_separators_to_add} separators to have {n_broken_groups} broken groups");
        // we count the number of broken elements we need to add
        let n_broken_elems: usize = second.split(',').map(|x| x.parse::<usize>().unwrap()).sum();
        let mut n_broken_to_add: usize = n_broken_elems - broken_groups.iter().map(|(_, x)| x).sum::<usize>();
        //println!("We need to add {n_broken_to_add} broken elems to have {n_broken_elems} broken elems");
        //println!("{:?}", unknown_groups);
        let mut free_slots = unknown_groups.iter().map(|(_, length)| length).sum::<usize>();
        /*println!(
            "We need to add {} elems and we have {} free slots",
            n_broken_to_add + n_separators_to_add,
            free_slots
        );*/
        if free_slots == n_separators_to_add + n_broken_to_add {
            valid_combinations += 1;
            continue;
        }
        let mut index_unkown = 0;
        let mut index_broken = 0;
        let mut partial_sequence: Vec<(Cell, usize, usize)> = Vec::new();
        let mut broken_count_iterator = second.split(',').map(|x| x.parse::<usize>().unwrap()).enumerate();
        let mut next_iterator = broken_count_iterator.next();
        while next_iterator.is_some() {
            let (i, broken_count) = next_iterator.unwrap();
        
        //for (i, broken_count) in second.split(',').map(|x| x.parse::<usize>().unwrap()).enumerate() {
            // get the first unknown group
            if index_unkown >= unknown_groups.len() {
                // we don't have unknown so we can go over
                //println!("break missing unkown");
                while index_broken < broken_groups.len() {
                    partial_sequence.push((Cell::BROKEN, broken_groups[index_broken].0, broken_groups[index_broken].1));
                    index_broken += 1;
                }
                break;
            }
            let ug = unknown_groups[index_unkown];
            if index_broken >= broken_groups.len() {
                // we don't have broken groups so we can go over
                //println!("break missing broken");
                while index_unkown < unknown_groups.len() {
                    partial_sequence.push((Cell::UNKOWN, unknown_groups[index_unkown].0, unknown_groups[index_unkown].1));
                    index_unkown += 1;
                }
                break;
            }
            let bg = broken_groups[index_broken];
            //let bg = broken_groups.get_mut(index_broken).unwrap();

            // check if they are contiguous
            if (bg.0 > ug.0) & ((ug.0 + ug.1).abs_diff(bg.0) == 0) {
                // check if bg is already complete. If so then put all the unknown to .
                if bg.1 == broken_count {
                    // the entire unkown group is useless, so we add a new group that is of separators
                    partial_sequence.push((Cell::SEPARATOR, ug.0, ug.1));
                    free_slots -= ug.1;
                    partial_sequence.push((Cell::BROKEN, bg.0, bg.1));
                    index_unkown += 1;
                    index_broken += 1;
                    // if we have an unknown after our broken, the first element must be .
                    if index_unkown < unknown_groups.len()
                    {if unknown_groups[index_unkown].0 == bg.0 + bg.1 {
                        let ug = unknown_groups.get_mut(index_unkown).unwrap();
                        partial_sequence.push((Cell::SEPARATOR, ug.0, 1));
                        free_slots -= 1;
                        if ug.1 > 1 {
                            ug.0 += 1;
                            ug.1 -= 1;
                        } else {   
                            index_unkown += 1;
                        }
                    }}
                    next_iterator = broken_count_iterator.next();
                } else {
                    let missing_broken = broken_count - bg.1;
                    if missing_broken > ug.1 {
                        // in this case, this group of unknown does not complete the group of broken
                        // therefore, we update bg with these new unknown but we need to repeat the loop
                        let bg = broken_groups.get_mut(index_broken).unwrap();
                        bg.0 = ug.0;
                        bg.1 += ug.1;
                        index_unkown += 1;
                        n_broken_to_add -= ug.1;
                        free_slots -= ug.1;
                        next_iterator = broken_count_iterator.next();
                        continue;
                    } else if ug.1 - missing_broken > 0 {
                        // in this case we add the separators
                        partial_sequence.push((Cell::SEPARATOR, ug.0, ug.1 - missing_broken));
                    } 
                    partial_sequence.push((Cell::BROKEN, bg.0 - missing_broken, bg.1 + missing_broken));
                    n_broken_to_add -= missing_broken;
                    free_slots -= missing_broken;
                    index_unkown += 1;
                    index_broken += 1;
                    next_iterator = broken_count_iterator.next();
                }
            } else if (ug.0 > bg.0) & ((ug.0).abs_diff(bg.0 + bg.1) == 0) {
                if bg.1 == broken_count {
                    // the first element of unkown is set to separator but we don't know if the next ones are needed
                    partial_sequence.push((Cell::BROKEN, bg.0, bg.1));
                    partial_sequence.push((Cell::SEPARATOR, ug.0, 1));
                    free_slots -= 1;
                    if ug.1 > 1 {
                        unknown_groups.push((ug.0 + 1, ug.1 - 1));
                        unknown_groups.sort_by(|a, b| a.0.cmp(&b.0))
                    }
                    index_unkown += 1;
                    index_broken += 1;
                    next_iterator = broken_count_iterator.next();
                } else {
                    let missing_broken = broken_count - bg.1;
                    if ug.1 == missing_broken {
                        // we transform all the unkown into broken
                        partial_sequence.push((Cell::BROKEN, bg.0, bg.1 + ug.1));
                        index_unkown += 1;
                        index_broken += 1;
                        n_broken_to_add -= missing_broken;
                        free_slots -= missing_broken;
                        next_iterator = broken_count_iterator.next();
                    } else if ug.1 < missing_broken {
                        // we expand the current partial broken group with this unknown but
                        // we need to wait the second loop
                        let bg = broken_groups.get_mut(index_broken).unwrap();
                        bg.1 += ug.1;
                        index_unkown += 1;
                        n_broken_to_add -= ug.1;
                        free_slots -= ug.1;
                        //next_iterator = broken_count_iterator.next();
                    } else {
                        // we take the unknown to complete the broken group
                        // then we put the next one as separator and the remaining as unknwon
                        partial_sequence.push((Cell::BROKEN, bg.0, missing_broken + bg.1));
                        partial_sequence.push((Cell::SEPARATOR, ug.0 + missing_broken, 1));
                        if ug.1 - missing_broken - 1 > 0 {
                            unknown_groups.push((ug.0 + 1, ug.1 - missing_broken - 1));
                            unknown_groups.sort_by(|a, b| a.0.cmp(&b.0))
                        }
                        index_broken += 1;
                        n_broken_to_add -= missing_broken;
                        free_slots -= missing_broken + 1;
                        next_iterator = broken_count_iterator.next();
                    }
                    
                }
            } else if bg.0 > ug.0 {
                if ug.1 == broken_count {
                    // sine we have the same number of ? we set all of them to #
                    partial_sequence.push((Cell::BROKEN, ug.0, ug.1));
                    n_broken_to_add -= ug.1;
                    free_slots -= ug.1;
                    next_iterator = broken_count_iterator.next();
                } else {
                    // we put again the unknown because mutliple solution can be possible
                    partial_sequence.push((Cell::UNKOWN, ug.0, ug.1));
                    index_unkown += 1;
                    next_iterator = broken_count_iterator.next();
                }
            } else {
                // ug.0 > bg.0
                if broken_count == bg.1 {
                    partial_sequence.push((Cell::BROKEN, bg.0, bg.1));
                    index_broken += 1;
                    next_iterator = broken_count_iterator.next();
                } else {
                    let next_bg = broken_groups.get_mut(index_broken + 1).unwrap();
                    next_bg.0 = bg.0;
                    next_bg.1 += bg.1;
                    index_broken += 1;
                }
            }
        }
        for sg in separator_groups {
            partial_sequence.push((Cell::SEPARATOR, sg.0, sg.1));
        }
        partial_sequence.sort_by(|a, b| a.1.cmp(&b.1));

        let mut partial_sequence_string: String = String::new();
        for (cell_elem, start_elem, len_elem) in partial_sequence {
            match cell_elem {
                Cell::BROKEN => {
                    for _ in 0..len_elem {
                        partial_sequence_string.push('#');
                    }
                }
                Cell::SEPARATOR => {
                    for _ in 0..len_elem {
                        partial_sequence_string.push('.');
                    }
                }
                Cell::UNKOWN => {
                    for _ in 0..len_elem {
                        partial_sequence_string.push('?');
                    }
                }
            }
        }
        let first = partial_sequence_string;
        if (free_slots == 0) & (n_broken_to_add == 0) {
            valid_combinations += 1;
            continue;
        }
        //let combinations = (0..free_slots).permutations(n_broken_to_add + n_separators_to_add).collect::<Vec<Vec<usize>>>();
        let mut combinations: Vec<Vec<usize>> = (0..free_slots).combinations(n_broken_to_add).collect::<Vec<Vec<usize>>>();
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
    let mut file = File::open("inputs/day_12.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut valid_combinations = 0;
    for (line_id, line) in lines.iter().enumerate() {
        println!("Line {line_id} / {}", lines.len());
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