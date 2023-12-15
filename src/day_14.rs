use std::{fs::File, io::Read};

use ndarray::{Array2, Axis, arr2, Array};
use indicatif::{ProgressBar, ProgressIterator};
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

/// Get the matrix, the position of round rocks and of the square rocks
/// it returns the new array and the new position of round rocks and load
fn move_north(mut array: Array2<i32>, round_rocks: Vec<(usize, usize)>, square_rocks: &Vec<(usize, usize)>) -> (Array2<i32>, Vec<(usize, usize)>, usize) {
    array.accumulate_axis_inplace(Axis(0), |&prev, curr| *curr += prev);
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
    }
    let mut new_array: Array2<i32> = Array2::zeros((array.shape()[0], array.shape()[1]));
    for rock in round_rock_positions.iter() {
        new_array[*rock] = 1;
    }
    for rock in square_rocks {
        new_array[*rock] = 1;
    }
    (new_array, round_rock_positions, load)
}

fn print_matrix(array: &Array2<i32>, round_rocks: &Vec<(usize, usize)>, square_rocks: &Vec<(usize, usize)>) {
    for i in 0..array.shape()[0] {
        println!();
        for j in 0..array.shape()[1] {
            if round_rocks.contains(&(i, j)) {
                print!("O");
            } else if square_rocks.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }

        }
    }
    println!();

    println!();

}

fn matrix_with_int(array: &Array2<i32>, round_rocks: &Vec<(usize, usize)>, square_rocks: &Vec<(usize, usize)>)  -> Array2<i32> {
    let mut result: Array2<i32> = Array2::zeros((array.shape()[0], array.shape()[1]));

    for i in 0..array.shape()[0] {
        for j in 0..array.shape()[1] {
            if round_rocks.contains(&(i, j)) {
                result[(i, j)] = 2;
            } else if square_rocks.contains(&(i, j)) {
                result[(i, j)] = 1;
            } else {
                result[(i, j)] = 0;
            }
        }
    }
    result
}

fn parse_int_matrix(array: &Array2<i32>) -> (Array2<i32>, Vec<(usize, usize)>, Vec<(usize, usize)>){
    let mut result: Array2<i32> = Array2::zeros((array.shape()[0], array.shape()[1]));
    let mut round_rocks: Vec<(usize, usize)> = Vec::new();
    let mut square_rocks: Vec<(usize, usize)> = Vec::new();
    for i in 0..array.shape()[0] {
        for j in 0..array.shape()[1] {
            if (array[(i, j)] == 1) | (array[(i, j)] == 2) {
                result[(i, j)] = 1;
                if array[(i, j)] == 2 {
                    round_rocks.push((i, j));
                } else {
                    square_rocks.push((i, j));
                }
            }
        }
    }
    (result, round_rocks, square_rocks)
}

fn rotate(array: &Array2<i32>, mut round_rocks: Vec<(usize, usize)>, mut square_rocks: Vec<(usize, usize)>) -> (Array2<i32>, Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut west_array: Array2<i32> = Array2::zeros((array.shape()[1], array.shape()[0]));
        for row in 0..array.shape()[0] {
            for col in 0..array.shape()[1] {
                west_array[(array.shape()[1] - 1 - col, array.shape()[0] - 1 - row)] = array[(row, col)];
            }
        }
        round_rocks = round_rocks.into_iter().map(|(row, col)| (array.shape()[1] - 1 - col, array.shape()[0] - 1 - row)).collect::<Vec<(usize, usize)>>();
        square_rocks = square_rocks.into_iter().map(|(row, col)| (array.shape()[1] - 1 - col, array.shape()[0] - 1 - row)).collect::<Vec<(usize, usize)>>();
    (west_array, round_rocks, square_rocks)
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_14.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (mut array, mut round_rocks, mut square_rocks) = parse_to_array(&contents);
    let mut load = 0;
    let n_cycles = 1000000000;
    let mut prev_load = 0;
    
    for _ in (0..n_cycles).progress_with(ProgressBar::new(n_cycles)) {
        // NORTH
        //println!("North");
        //print_matrix(&array, &round_rocks, &square_rocks);
        (array, round_rocks, _) = move_north(array, round_rocks, &square_rocks);
        // 0 is .
        // 1 is #
        // 2 is O
        let mut int_matrix = matrix_with_int(&array, &round_rocks, &square_rocks);
        // WEST
        int_matrix = int_matrix.reversed_axes();
        (array, round_rocks, square_rocks) = parse_int_matrix(&int_matrix);
        (array, round_rocks, _) = move_north(array, round_rocks, &square_rocks);
        let mut int_matrix = matrix_with_int(&array, &round_rocks, &square_rocks);
        // SOUTH
        int_matrix.invert_axis(Axis(1));
        int_matrix = int_matrix.reversed_axes();
        (array, round_rocks, square_rocks)= parse_int_matrix(&int_matrix);
        (array, round_rocks, _) = move_north(array, round_rocks, &square_rocks);
        let mut int_matrix = matrix_with_int(&array, &round_rocks, &square_rocks);
        // EAST
        int_matrix.invert_axis(Axis(1));
        int_matrix = int_matrix.reversed_axes();
        int_matrix.invert_axis(Axis(1));
        (array, round_rocks, square_rocks) = parse_int_matrix(&int_matrix);
        (array, round_rocks, load) = move_north(array, round_rocks, &square_rocks);
        if load == prev_load {
            break;
        }
        let mut int_matrix = matrix_with_int(&array, &round_rocks, &square_rocks);
        // NORTH
        int_matrix.invert_axis(Axis(1));
        int_matrix = int_matrix.reversed_axes();
        int_matrix.invert_axis(Axis(1));
        int_matrix.invert_axis(Axis(0));
        (array, round_rocks, square_rocks)= parse_int_matrix(&int_matrix);
        //println!("load is {load} and prev is {prev_load}");
        //print_matrix(&array, &round_rocks, &square_rocks);
        //println!();
        prev_load = load;
    }
    println!("load is {load}");
}

pub fn solve_pt2_old() {
    let mut file = File::open("inputs/day_14_example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (mut array, mut round_rocks, mut square_rocks) = parse_to_array(&contents);
    let mut load = 0;
    let n_cycles = 1;//1000000000;
    
    for _ in 0..n_cycles {
        // (array, round_rocks, load) = move_north(array, round_rocks, &square_rocks);
        // North
        println!("North");
        print_matrix(&array, &round_rocks, &square_rocks);
        (array, round_rocks, load) = move_north(array, round_rocks, &square_rocks);
        println!("North AFTER");
        print_matrix(&array, &round_rocks, &square_rocks);
        // West
        /*let mut west_array: Array2<i32> = Array2::zeros((array.shape()[1], array.shape()[0]));
        for row in 0..array.shape()[0] {
            for col in 0..array.shape()[1] {
                west_array[(array.shape()[1] - 1 - col, array.shape()[0] - 1 - row)] = array[(row, col)];
            }
        }
        round_rocks = round_rocks.into_iter().map(|(row, col)| (col, array.shape()[0] - 1 - row)).collect::<Vec<(usize, usize)>>();
        square_rocks = square_rocks.into_iter().map(|(row, col)| (col, array.shape()[0] - 1 - row)).collect::<Vec<(usize, usize)>>();
        array = west_array;*/
        (array, round_rocks, square_rocks) = rotate(&array, round_rocks, square_rocks);
        
        println!("West");
        print_matrix(&array, &round_rocks, &square_rocks);
        (array, round_rocks, load) = move_north(array, round_rocks, &square_rocks);
        println!("West AFTER");
        print_matrix(&array, &round_rocks, &square_rocks);

        /*let mut south_array: Array2<i32> = Array2::zeros((array.shape()[1], array.shape()[0]));
        for row in 0..array.shape()[0] {
            for col in 0..array.shape()[1] {
                south_array[(array.shape()[1] - 1 - col, row)] = array[(row, col)];
            }
        }
        round_rocks = round_rocks.into_iter().map(|(row, col)| (array.shape()[1] - 1 - col, row)).collect::<Vec<(usize, usize)>>();
        square_rocks = square_rocks.into_iter().map(|(row, col)| (array.shape()[1] - 1 - col, row)).collect::<Vec<(usize, usize)>>();
        array = south_array;*/
        (array, round_rocks, square_rocks) = rotate(&array, round_rocks, square_rocks);
        println!("South");
        print_matrix(&array, &round_rocks, &square_rocks);
        (array, round_rocks, load) = move_north(array, round_rocks, &square_rocks);
        println!("South AFTER");
        print_matrix(&array, &round_rocks, &square_rocks);

        /*let mut east_array: Array2<i32> = Array2::zeros((array.shape()[1], array.shape()[0]));
        for row in 0..array.shape()[0] {
            for col in 0..array.shape()[1] {
                east_array[(col, row)] = array[(array.shape()[0] - 1 - row, array.shape()[1] - 1 - col)];
            }
        }
        round_rocks = round_rocks.into_iter().map(|(row, col)| (array.shape()[0] - 1 - row, array.shape()[1] - 1 - col)).collect::<Vec<(usize, usize)>>();
        square_rocks = square_rocks.into_iter().map(|(row, col)| (array.shape()[0] - 1 - row, array.shape()[1] - 1 - col)).collect::<Vec<(usize, usize)>>();
        array = east_array;*/
        (array, round_rocks, square_rocks) = rotate(&array, round_rocks, square_rocks);
        println!("East");
        print_matrix(&array, &round_rocks, &square_rocks);
        (array, round_rocks, load) = move_north(array, round_rocks, &square_rocks);
        println!("East AFTER");
        print_matrix(&array, &round_rocks, &square_rocks);
    }
    println!("The final load is {load}");
}

/*

north
123
456

west
14
25
36

south
456
123

east
36
25
14



*/