use std::{fs::File, io::Read};

use geo::{Contains, line_string};
use geo::{Polygon, LineString, point};

#[derive(PartialEq, Debug)]
enum Direction {
    NONE,
    LEFT,
    RIGHT,
    UP,
    DOWN
}
struct Pipeline {
    start_index: usize,
    map: String,
    map_dim: (usize, usize)
}

impl Pipeline {
    fn new(map: String) -> Pipeline {
        let map_dim = (map.lines().collect::<Vec<&str>>().len(), map.lines().collect::<Vec<&str>>().get(0).unwrap().len());
        let start_index = map.replace("\n", "").find("S").unwrap();
        Pipeline { 
            start_index, 
            map: map.replace("\n", ""), 
            map_dim 
        }
    }

    fn to_coordinates(&self, index: &usize) -> (f64, f64) {
        let row = (*index as f64 / self.map_dim.1 as f64).floor();
        let col = *index as f64 - row * self.map_dim.1 as f64;
        (row, col)
    }

    fn get_symbol(&self, position: &usize) -> char {
        self.map.chars().nth(*position).unwrap()
    }

    fn look_up(&self, current_position: &usize) -> Option<usize> {
        if ['|', 'S', 'J', 'L'].contains(&self.get_symbol(current_position)) {
            // if we are not at the first row
            if *current_position >= self.map_dim.1 {
                let new_position = current_position - self.map_dim.1;
                if ['|', 'F', '7', 'S'].contains(&self.map.chars().nth(new_position).unwrap()) {
                    return Some(new_position);
                }
                return None;
            }
            return None;
        }
        None
    }

    fn look_down(&self, current_position: &usize) -> Option<usize> {
        if ['|', 'S', 'F', '7'].contains(&self.get_symbol(current_position)) {
        // if we are not at the last row
        if *current_position < self.map_dim.1 * (self.map_dim.0 - 1) {
            let new_position = current_position + self.map_dim.1;
            if ['|', 'L', 'J', 'S'].contains(&self.map.chars().nth(new_position).unwrap()) {
                return Some(new_position);
            }
            return None;
        }
        return None;
    }
        None
    }

    fn look_left(&self, current_position: &usize) -> Option<usize> {
        if ['-', 'S', 'J', '7'].contains(&self.get_symbol(current_position)) {
            // if we are not at the first column
            if (current_position % self.map_dim.1) != 0 {
                let new_position = current_position - 1;
                if ['F', 'L', 'S', '-'].contains(&self.map.chars().nth(new_position).unwrap()) {
                    return Some(new_position);
                }
                return None;
            }
            return None;
        }
        None
    }

    fn look_right(&self, current_position: &usize) -> Option<usize> {
        if ['-', 'S', 'F', 'L'].contains(&self.get_symbol(current_position)) {
        // if we are not at the first column
        if ((current_position + 1) % self.map_dim.1) != 0 {
            let new_position = current_position + 1;
            if ['J', '7', 'S', '-'].contains(&self.map.chars().nth(new_position).unwrap()) {
                return Some(new_position);
            }
            return None;
        }
        return None;
    }
        None
    }

    fn make_a_step(&self, current_position: &usize, prev_direction: &Direction) -> (usize, Direction) {
        if let Some(next_position) = self.look_down(current_position) {
            if *prev_direction != Direction::UP {
                return (next_position, Direction::DOWN);
            }
        }
        if let Some(next_position) = self.look_up(current_position) {
            if *prev_direction != Direction::DOWN {
                return (next_position, Direction::UP);
            }
        }
        if let Some(next_position) = self.look_left(current_position) {
            if *prev_direction != Direction::RIGHT {
                return (next_position, Direction::LEFT);
            }
        }
        if let Some(next_position) = self.look_right(current_position) {
            if *prev_direction != Direction::LEFT {
                return (next_position, Direction::RIGHT);
            }
        }
        panic!()
    }
}


pub fn solve() {
    let mut file = File::open("inputs/day_10.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let pipeline = Pipeline::new(contents);
    /*for pos in 0..25 {
        println!("POS {pos}");
        println!("Up {:?}", pipeline.look_up(&pos));
        println!("Left {:?}", pipeline.look_left(&pos));
        println!("Down {:?}", pipeline.look_down(&pos));
        println!("Right {:?}", pipeline.look_right(&pos));
        println!()
    }*/
    let mut steps: Vec<usize> = Vec::new();
    let mut seen_positions: Vec<usize> = Vec::new();
    let (mut current_position , mut direction) = pipeline.make_a_step(&pipeline.start_index, &Direction::NONE);
    seen_positions.push(current_position);
    println!("STEP {:?}: {} -> {current_position}", direction, pipeline.start_index);
    steps.push(current_position);
    while current_position != pipeline.start_index {
        let (next_position, next_direction) = pipeline.make_a_step(&current_position, &direction);
        println!("STEP {:?}: {current_position} -> {next_position}", next_direction);
        current_position = next_position;
        direction = next_direction;
        steps.push(current_position);
        if seen_positions.contains(&current_position) {
            println!("ops");
            break;
        }
        seen_positions.push(current_position);
    }
    println!("Steps are {:?}", steps);
    println!("Farthest step is {}", steps.len() / 2);
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_10.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let pipeline = Pipeline::new(contents);
    let mut steps: Vec<usize> = Vec::new();
    let mut seen_positions: Vec<usize> = Vec::new();
    
    let (mut current_position , mut direction) = pipeline.make_a_step(&pipeline.start_index, &Direction::NONE);
    seen_positions.push(current_position);
    //println!("STEP {:?}: {} -> {current_position}", direction, pipeline.start_index);
    steps.push(current_position);
    while current_position != pipeline.start_index {
        let (next_position, next_direction) = pipeline.make_a_step(&current_position, &direction);
        //println!("STEP {:?}: {current_position} -> {next_position}", next_direction);
        current_position = next_position;
        direction = next_direction;
        steps.push(current_position);
        if seen_positions.contains(&current_position) {
            println!("ops");
            break;
        }
        seen_positions.push(current_position);
    }
    //println!("Steps are {:?}", steps);
    // build loop polygon and look for points inside it
    let mut coordinates_vec: Vec<(f64, f64)> = Vec::new();
    for point in steps {
        coordinates_vec.push(pipeline.to_coordinates(&point));
    }
    let polygon = Polygon::new(
        LineString::from(coordinates_vec),
        vec![]
    );
    //println!("polygon {:?}", polygon);
    
    let mut inside_points: Vec<(usize, usize)> = Vec::new();
    for r in 0..pipeline.map_dim.0 {
        for c in 0..pipeline.map_dim.1 {
            if polygon.contains(&point!(x: r as f64, y: c as f64)) {
                inside_points.push((r, c));
            }
        }
    }
    println!("there are {} inside points", inside_points.len());

}