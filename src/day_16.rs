use std::{fs::File, io::Read};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
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
        let map_dim = (
            map.lines()
            .collect::<Vec<&str>>()
            .len(), 
            map.lines()
            .collect::<Vec<&str>>()
            .get(0).unwrap().len()
        );

        let start_index = 0;
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

    fn get_id(&self, row: &usize, col: &usize) -> usize {
        row * self.map_dim.1 + col
    }

    fn look_up(&self, current_position: &usize) -> Option<usize> {
            // if we are not at the first row
            if *current_position >= self.map_dim.1 {
                let new_position = current_position - self.map_dim.1;
                
                Some(new_position)
                
        } else {
            None
        }
    }

    fn look_down(&self, current_position: &usize) -> Option<usize> {
        // if we are not at the last row
        if *current_position < self.map_dim.1 * (self.map_dim.0 - 1) {
            let new_position = current_position + self.map_dim.1;
                Some(new_position)
        } else{
            None
        }
    }

    fn look_left(&self, current_position: &usize) -> Option<usize> {
            // if we are not at the first column
        if (current_position % self.map_dim.1) != 0 {
            let new_position = current_position - 1;
            Some(new_position)
        } else {
            None
        }
    }

    fn look_right(&self, current_position: &usize) -> Option<usize> {
        // if we are not at the first column
        if ((current_position + 1) % self.map_dim.1) != 0 {
            let new_position = current_position + 1;
            Some(new_position)
        } else {
            None
        }
    }

    fn make_a_step(&self, current_position: &usize, prev_direction: &Direction) -> Vec<(usize, Direction)> {
        match prev_direction {
            &Direction::RIGHT => {
                // we go straight if we encounter . or -
                if ['.', '-'].contains(&self.get_symbol(current_position)) {
                    if let Some(new_position) = self.look_right(current_position) {
                        return vec![(new_position, Direction::RIGHT)];
                    } else {
                        return vec![];
                    }
                } else if '|' == self.get_symbol(current_position) {
                    // we go up or down according to if we are in the middle or at one edge
                    let mut new_positions: Vec<(usize, Direction)> = Vec::new();
                    if let Some(new_position) = self.look_down(current_position) {
                        new_positions.push((new_position, Direction::DOWN));
                    }
                    if let Some(new_position) = self.look_up(current_position) {
                        new_positions.push((new_position, Direction::UP));
                    }
                    return new_positions;
                } else if '\\' == self.get_symbol(current_position) {
                    if let Some(new_position) = self.look_down(current_position) {
                        return vec![(new_position, Direction::DOWN)];
                    } else {
                        return vec![];
                    }
                } else if '/' == self.get_symbol(current_position) {
                    if let Some(new_position) = self.look_up(current_position) {
                        return vec![(new_position, Direction::UP)];
                    } else {
                        return vec![];
                    }
                } else {
                    return vec![];
                }
            },
            &Direction::DOWN => {
                // we go straight if we encounter . or -
                if ['.', '|'].contains(&self.get_symbol(current_position)) {
                    if let Some(new_position) = self.look_down(current_position) {
                        return vec![(new_position, Direction::DOWN)];
                    } else {
                        return vec![];
                    }
                } else if '-' == self.get_symbol(current_position) {
                    // split right and left
                    let mut new_positions: Vec<(usize, Direction)> = Vec::new();
                    if let Some(new_position) = self.look_left(current_position) {
                        new_positions.push((new_position, Direction::LEFT));
                    }
                    if let Some(new_position) = self.look_right(current_position) {
                        new_positions.push((new_position, Direction::RIGHT));
                    }
                    return new_positions;
                } else if '\\' == self.get_symbol(current_position) {
                    // go right
                    if let Some(new_position) = self.look_right(current_position) {
                        return vec![(new_position, Direction::RIGHT)];
                    } else {
                        return vec![];
                    }
                } else if '/' == self.get_symbol(current_position) {
                    if let Some(new_position) = self.look_left(current_position) {
                        return vec![(new_position, Direction::LEFT)];
                    } else {
                        return vec![];
                    }
                } else {
                    return vec![];
                }
            },
            &Direction::UP => {
                // we go straight if we encounter . or -
                if ['.', '|'].contains(&self.get_symbol(current_position)) {
                    if let Some(new_position) = self.look_up(current_position) {
                        return vec![(new_position, Direction::UP)];
                    } else {
                        return vec![];
                    }
                } else if '-' == self.get_symbol(current_position) {
                    // split right and left
                    let mut new_positions: Vec<(usize, Direction)> = Vec::new();
                    if let Some(new_position) = self.look_left(current_position) {
                        new_positions.push((new_position, Direction::LEFT));
                    }
                    if let Some(new_position) = self.look_right(current_position) {
                        new_positions.push((new_position, Direction::RIGHT));
                    }
                    return new_positions;
                } else if '\\' == self.get_symbol(current_position) {
                    if let Some(new_position) = self.look_left(current_position) {
                        return vec![(new_position, Direction::LEFT)];
                    } else {
                        return vec![];
                    }
                } else if '/' == self.get_symbol(current_position) {
                    // go right
                    if let Some(new_position) = self.look_right(current_position) {
                        return vec![(new_position, Direction::RIGHT)];
                    } else {
                        return vec![];
                    }
                } else {
                    return vec![];
                }
            },
            &Direction::LEFT => {
                // we go straight if we encounter . or -
                if ['.', '-'].contains(&self.get_symbol(current_position)) {
                    if let Some(new_position) = self.look_left(current_position) {
                        return vec![(new_position, Direction::LEFT)];
                    } else {
                        return vec![];
                    }
                } else if '|' == self.get_symbol(current_position) {
                    let mut new_positions: Vec<(usize, Direction)> = Vec::new();
                    if let Some(new_position) = self.look_down(current_position) {
                        new_positions.push((new_position, Direction::DOWN));
                    }
                    if let Some(new_position) = self.look_up(current_position) {
                        new_positions.push((new_position, Direction::UP));
                    }
                    return new_positions;
                } else if '\\' == self.get_symbol(current_position) {
                    if let Some(new_position) = self.look_up(current_position) {
                        return vec![(new_position, Direction::UP)];
                    } else {
                        return vec![];
                    }
                } else if '/' == self.get_symbol(current_position) {
                    if let Some(new_position) = self.look_down(current_position) {
                        return vec![(new_position, Direction::DOWN)];
                    } else {
                        return vec![];
                    }
                } else {
                    return vec![];
                }
            }
        }
    }
}

fn print_matrix(pipeline: &Pipeline, map_dim: &(usize, usize), seen_positions: &Vec<(usize)>) {
    for i in 0..map_dim.0 {
        println!();
        for j in 0..map_dim.1 {
            if seen_positions.contains(&pipeline.get_id(&i, &j)) {
                print!("O");
            } else {
                print!(".");
            }

        }
    }
    println!();
    println!();

}


pub fn solve() {
    let mut file = File::open("inputs/day_16.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let pipeline = Pipeline::new(contents);
    let mut energized_cells: Vec<usize> = Vec::new();
    let mut seen_positions: Vec<(usize, Direction)> = Vec::new();
    let mut ways = vec![(0, Direction::RIGHT)];
    while ways.len() > 0 {
        let mut new_ways: Vec<(usize, Direction)> = Vec::new();
        for way in ways.iter() {
            seen_positions.push(way.clone());
            //if ['-', '|', '\\', '/'].contains(&pipeline.get_symbol(&way.0)) & !energized_cells.contains(&way.0){
            if !energized_cells.contains(&way.0){
                energized_cells.push(way.0.clone());
            }
            let mut step = pipeline.make_a_step(&way.0, &way.1);
            new_ways.append(&mut step);
        }
        ways = new_ways.into_iter().filter(|x| !seen_positions.contains(x)).collect();
    }
    print_matrix(&pipeline, &pipeline.map_dim, &seen_positions.iter().map(|x| x.0).collect());
    println!("We energized {} cells", energized_cells.len());
}


fn make_run(initial_position: (usize, Direction), pipeline: &Pipeline) -> usize {
    let mut energized_cells: Vec<usize> = Vec::new();
    let mut seen_positions: Vec<(usize, Direction)> = Vec::new();
    let mut ways = vec![initial_position];
    while ways.len() > 0 {
        let mut new_ways: Vec<(usize, Direction)> = Vec::new();
        for way in ways.iter() {
            seen_positions.push(way.clone());
            //if ['-', '|', '\\', '/'].contains(&pipeline.get_symbol(&way.0)) & !energized_cells.contains(&way.0){
            if !energized_cells.contains(&way.0){
                energized_cells.push(way.0.clone());
            }
            let mut step = pipeline.make_a_step(&way.0, &way.1);
            new_ways.append(&mut step);
        }
        ways = new_ways.into_iter().filter(|x| !seen_positions.contains(x)).collect();
    }
    energized_cells.len()
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_16.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut max_energized_cells = 0;

    let pipeline = Pipeline::new(contents);


    // Angle up left
    let energized_cells = make_run((pipeline.get_id(&0, &0), Direction::RIGHT), &pipeline);            
    if energized_cells > max_energized_cells {
        max_energized_cells = energized_cells;
    }
    let energized_cells = make_run((pipeline.get_id(&0, &0), Direction::DOWN), &pipeline);            
    if energized_cells > max_energized_cells {
        max_energized_cells = energized_cells;
    }

    // Angle up right
    let energized_cells = make_run((pipeline.get_id(&0, &(pipeline.map_dim.1 - 1)), Direction::LEFT), &pipeline);
    if energized_cells > max_energized_cells {
        max_energized_cells = energized_cells;
    }
    let energized_cells = make_run((pipeline.get_id(&0, &(pipeline.map_dim.1 - 1)), Direction::DOWN), &pipeline);            
    if energized_cells > max_energized_cells {
        max_energized_cells = energized_cells;
    }

    // Angle bottom right
    let energized_cells = make_run((pipeline.get_id(&(pipeline.map_dim.0 - 1), &(pipeline.map_dim.1 - 1)), Direction::LEFT), &pipeline);
    if energized_cells > max_energized_cells {
        max_energized_cells = energized_cells;
    }
    let energized_cells = make_run((pipeline.get_id(&(pipeline.map_dim.0 - 1), &(pipeline.map_dim.1 - 1)), Direction::DOWN), &pipeline);            
    if energized_cells > max_energized_cells {
        max_energized_cells = energized_cells;
    }

    // Angle bottom left
    let energized_cells = make_run((pipeline.get_id(&(pipeline.map_dim.0 - 1), &0), Direction::RIGHT), &pipeline);
    if energized_cells > max_energized_cells {
        max_energized_cells = energized_cells;
    }
    let energized_cells = make_run((pipeline.get_id(&(pipeline.map_dim.0 - 1), &0), Direction::DOWN), &pipeline);            
    if energized_cells > max_energized_cells {
        max_energized_cells = energized_cells;
    }

    // First row
    for j in 0..pipeline.map_dim.1 {
        let energized_cells = make_run((pipeline.get_id(&0, &j), Direction::DOWN), &pipeline);            
        if energized_cells > max_energized_cells {
            max_energized_cells = energized_cells;
        }
    }

    // First column
    for i in 0..pipeline.map_dim.0 {
        let energized_cells = make_run((pipeline.get_id(&i, &0), Direction::RIGHT), &pipeline);            
        if energized_cells > max_energized_cells {
            max_energized_cells = energized_cells;
        }
    }

    // Last column
    for i in 0..pipeline.map_dim.0 {
        let energized_cells = make_run((pipeline.get_id(&i, &(pipeline.map_dim.1 - 1)), Direction::LEFT), &pipeline);            
        if energized_cells > max_energized_cells {
            max_energized_cells = energized_cells;
        }
    }

    // Last row
    for j in 0..pipeline.map_dim.1 {
        let energized_cells = make_run((pipeline.get_id(&(pipeline.map_dim.0 - 1), &j), Direction::DOWN), &pipeline);            
        if energized_cells > max_energized_cells {
            max_energized_cells = energized_cells;
        }
    }

    println!("We energized {} cells", max_energized_cells);
}
