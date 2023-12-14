use std::{fs::File, io::Read};

use ndarray::{Array2, Axis};

/// Returns the array, a vector of round rocks and a vector of square rocks
fn parse_to_array(block: &str) -> (Array2<i32>, Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let lines = block.split('\n').collect::<Vec<&str>>();
    let mut array = Array2::zeros((lines.len(), lines[0].len()));
    let mut round_rocks: Vec<(usize, usize)> = Vec::new();
    let mut square_rocks: Vec<(usize, usize)> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if (c == '#') | (c == 'O') {
                array[(i, j)] = 1;
                if c == 'O' {
                    round_rocks.push((i, j));
                } else {
                    square_rocks.push((i, j));
                }
            }
        }
    }
    (array, round_rocks, square_rocks)
}

pub fn solve() {
    let mut file = File::open("inputs/day_14.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (mut array, round_rocks, square_rocks) = parse_to_array(&contents);
    //println!("Array is\n{:?}", array);
    array.accumulate_axis_inplace(Axis(0), |&prev, curr| *curr += prev);
    /*println!("Array cumsum is\n{:?}", array);
    println!("Square rocks are\n{:?}", square_rocks);
    println!("Round rocks are\n{:?}", round_rocks);*/
    // the final position of a round rock is the cumsum value minus - 1 miuns the position of a square rock
    let mut round_rock_positions: Vec<(usize, usize)> = Vec::new();
    let mut load = 0;
    for rock in round_rocks.iter() {
        let cumsum = array[*rock];
        let mut blocking_square_rocks = square_rocks.iter().filter(|(i, j)| (j == &rock.1) & (i < &rock.0)).collect::<Vec<&(usize, usize)>>();
        blocking_square_rocks.sort_by(|a, b| a.0.cmp(&b.0));
        
        let blocking_square_rock = blocking_square_rocks.last();
        let blocking_round_rocks = round_rocks.iter().filter(
            |(i, j)| (j == &rock.1) & (i < &rock.0) & (i >= &blocking_square_rock.unwrap_or(&&(0,0)).0)
        ).collect::<Vec<&(usize, usize)>>();
        if let Some(blocking_square_rock_coords) = blocking_square_rock {
            // the position is the position of the blocking square rocks +1 and + number of rounding rocks
            round_rock_positions.push((blocking_square_rock_coords.0 + 1 + blocking_round_rocks.len(), rock.1))
        } else {
            round_rock_positions.push((cumsum as usize - 1, rock.1));
        }
        load += array.shape()[0] - round_rock_positions.last().unwrap().0;
        /*println!("Blocking square rock for rock {:?} are\n{:?}", rock, blocking_square_rock);
        println!("Blocking round rocks for rock {:?} are\n{:?}", rock, blocking_round_rocks);
        println!("Rock rolled from {:?} to {:?}", rock, round_rock_positions.last().unwrap());
        println!()*/
    }
    
    //println!("Round rocks positions after rolling\n{:?}", round_rock_positions);

    println!("The total load is {}", load);
}