use std::{fs::File, io::Read, collections::HashSet};

use ndarray::{Array2, Axis};

fn manhattan_distance(node_0: &(usize, usize), node_1: &(usize, usize)) -> usize {
    node_0.0.abs_diff(node_1.0) + node_0.1.abs_diff(node_1.1)
}


pub fn solve() {
    let mut file = File::open("inputs/day_11.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut galaxy_matrix: Array2<i32> = Array2::zeros((lines.len(), lines.get(0).unwrap().len()));
    for (i, line) in lines.iter().enumerate() {
        let ones = line.match_indices('#');
        for (col, _) in ones {
            galaxy_matrix[(i, col)] = 1;
        }
    }
    println!("Galaxy is \n{:?}", galaxy_matrix);

    let cols_to_expand = galaxy_matrix
        .sum_axis(Axis(0))
        .iter()
        .enumerate()
        .filter(|(i, x)| **x == 0)
        .map(|(i, x)| i)
        .collect::<Vec<usize>>();
    let rows_to_expand = galaxy_matrix
        .sum_axis(Axis(1))
        .iter()
        .enumerate()
        .filter(|(i, x)| **x == 0)
        .map(|(i, x)| i)
        .collect::<Vec<usize>>();


    let mut nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut galaxy_matrix: Array2<i32> = Array2::zeros((lines.len() + rows_to_expand.len(), lines.get(0).unwrap().len() + cols_to_expand.len()));
    for (i, line) in lines.iter().enumerate() {
        let expanded_rows_before = rows_to_expand.iter().filter(|x| **x <= i).collect::<Vec<&usize>>().len();
        let ones = line.match_indices('#');
        for (col, _) in ones {
            let expanded_cols_before = cols_to_expand.iter().filter(|x| **x <= col).collect::<Vec<&usize>>().len();
            nodes.insert((i + expanded_rows_before, col + expanded_cols_before));
            galaxy_matrix[(i + expanded_rows_before, col + expanded_cols_before)] = 1;
        }
    }
    println!("Expanded galaxy is \n{:?}", galaxy_matrix);
    println!("Nodes are \n{:?}", nodes);

    /*let mut path_lengths: Vec<usize> = Vec::new();
    let mut matched_nodes: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    for node in nodes.iter() {
        let mut distance: Option<usize> = None;
        let mut matched_node: Option<(usize, usize)> = None;
        // the node is already matched, therefore we do not consider it
            for other_node in nodes.iter() {
                if other_node != node {
                    let other_distance = manhattan_distance(node, other_node);
                    if let Some(current_distance) = distance {
                        if current_distance < other_distance {
                            distance = Some(other_distance);
                            matched_node = Some(other_node.clone());
                        }
                    } else {
                        distance = Some(other_distance);
                        matched_node = Some(other_node.clone());
                    }
                }
            }
            let pair = (*node, matched_node.unwrap());
            let pair_reverse = (matched_node.unwrap(), *node);
            if !matched_nodes.contains(&pair) & !matched_nodes.contains(&pair_reverse) {
                matched_nodes.insert(pair);
                path_lengths.push(distance.unwrap());
            }
    }*/
    let nodes_vec = nodes.iter().collect::<Vec<&(usize, usize)>>();
    let mut path_lengths: Vec<usize> = Vec::new();
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let node = nodes_vec.get(i).unwrap();
            let other_node = nodes_vec.get(j).unwrap();
            path_lengths.push(manhattan_distance(node, other_node));
        }
    }
    println!("Path lengths {:?}", path_lengths);
    println!("Path lengths sum {:?}", path_lengths.iter().sum::<usize>());
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_11.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut galaxy_matrix: Array2<i32> = Array2::zeros((lines.len(), lines.get(0).unwrap().len()));
    for (i, line) in lines.iter().enumerate() {
        let ones = line.match_indices('#');
        for (col, _) in ones {
            galaxy_matrix[(i, col)] = 1;
        }
    }
    println!("Galaxy is \n{:?}", galaxy_matrix);

    let cols_to_expand = galaxy_matrix
        .sum_axis(Axis(0))
        .iter()
        .enumerate()
        .filter(|(i, x)| **x == 0)
        .map(|(i, x)| i)
        .collect::<Vec<usize>>();
    let rows_to_expand = galaxy_matrix
        .sum_axis(Axis(1))
        .iter()
        .enumerate()
        .filter(|(i, x)| **x == 0)
        .map(|(i, x)| i)
        .collect::<Vec<usize>>();
    println!("rows to expand are {:?} and cols to expad are {:?}", rows_to_expand, cols_to_expand);
    let offset = 1000000;
    let mut nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut galaxy_matrix: Array2<i32> = Array2::zeros((lines.len() + rows_to_expand.len() * (offset - 1), lines.get(0).unwrap().len() + cols_to_expand.len() * (offset - 1)));
    println!("galaxy matrix shape is {:?}", galaxy_matrix.shape());
    for (i, line) in lines.iter().enumerate() {
        let expanded_rows_before = rows_to_expand.iter().filter(|x| **x <= i).collect::<Vec<&usize>>().len() * (offset - 1);
        
        let ones = line.match_indices('#');
        for (col, _) in ones {
            let expanded_cols_before = cols_to_expand.iter().filter(|x| **x <= col).collect::<Vec<&usize>>().len() * (offset - 1);
            println!("Adding node from ({}, {}) to ({}, {})", i, col, i + expanded_rows_before, col + expanded_cols_before);
            nodes.insert((i + expanded_rows_before, col + expanded_cols_before));
            galaxy_matrix[(i + expanded_rows_before, col + expanded_cols_before)] = 1;
        }
    }
    println!("Expanded galaxy is \n{:?}", galaxy_matrix);
    println!("Nodes are \n{:?}", nodes);

    let nodes_vec = nodes.iter().collect::<Vec<&(usize, usize)>>();
    let mut path_lengths: Vec<usize> = Vec::new();
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let node = nodes_vec.get(i).unwrap();
            let other_node = nodes_vec.get(j).unwrap();
            path_lengths.push(manhattan_distance(node, other_node));
        }
    }
    println!("Path lengths {:?}", path_lengths);
    println!("Path lengths sum {:?}", path_lengths.iter().sum::<usize>());
}