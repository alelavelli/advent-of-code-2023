use std::{fs::File, io::Read};

fn is_a_symbol(c: char) -> bool {
    !c.is_ascii_digit() & (c != '.')
}

#[derive(Debug)]
struct Number {
    line_id: usize,
    columns: Vec<usize>,
    content: String,
}

pub fn solve() {
    let mut file = File::open("inputs/day_03.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    // vector containing found numbers
    let mut numbers: Vec<Number> = Vec::new();
    // vector containing coordinates of the symbols
    let mut symbols: Vec<(usize, usize)> = Vec::new();

    for (line_id, line) in lines.iter().enumerate() {
        let mut number: Option<Number> = None;

        for (char_id, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                // we found a number and now we look for its end and symbols
                if let Some(num) = number.as_mut() {
                    num.content.push(c);
                    num.columns.push(char_id);
                } else {
                    number = Some(Number {
                        line_id,
                        columns: vec![char_id],
                        content: String::from(c),
                    });
                }
            } else {
                if number.is_some() {
                    // the number is finished, we add it to the list of numbers
                    numbers.push(number.unwrap());
                    number = None;
                } else {
                    // nothing to do, we go on
                }

                if is_a_symbol(c) {
                    // if we found a symbol, then we save its coordinates
                    symbols.push((line_id, char_id));
                }
            }
        }
        if number.is_some() {
            numbers.push(number.unwrap());
        }
    }

    // now that we know the numbers and the symbols we verify if a number is near to a symbol
    let mut part_numbers_sum = 0;
    let mut part_numbers: Vec<&Number> = Vec::new();
    for number in &numbers {
        let mut is_part_number = false;
        // look upper line
        if number.line_id > 0 {
            let start = (*number.columns.first().unwrap() as i32 - 1).max(0) as usize;
            let end = (*number.columns.last().unwrap() + 1).min(lines[0].len() - 1);
            let line: &str = &contents.lines().nth(number.line_id - 1).unwrap()[start..=end];
            if line
                .chars()
                .filter(|x| is_a_symbol(*x))
                .collect::<Vec<char>>()
                .len()
                > 0
            {
                is_part_number = true;
            }
        }

        // look lower line
        if number.line_id < lines.len() - 1 {
            let start = (*number.columns.first().unwrap() as i32 - 1).max(0) as usize;
            let end = (*number.columns.last().unwrap() + 1).min(lines[0].len() - 1);
            let line: &str = &contents.lines().nth(number.line_id + 1).unwrap()[start..=end];
            if line
                .chars()
                .filter(|x| is_a_symbol(*x))
                .collect::<Vec<char>>()
                .len()
                > 0
            {
                is_part_number = true;
            }
        }

        // look left
        let first_col = *number.columns.first().unwrap();
        if first_col > 0 {
            if is_a_symbol(
                contents
                    .lines()
                    .nth(number.line_id)
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>()[first_col - 1],
            ) {
                is_part_number = true;
            }
        }

        // look right
        let last_col = *number.columns.last().unwrap();
        if last_col < lines[0].len() - 1 {
            if is_a_symbol(
                contents
                    .lines()
                    .nth(number.line_id)
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>()[last_col + 1],
            ) {
                is_part_number = true;
            }
        }

        if is_part_number {
            part_numbers_sum += number.content.parse::<i32>().unwrap();
            part_numbers.push(&number);
        }
    }
    //println!("Numbers are {:#?}", numbers.iter().map(|x| x.content.clone()).collect::<Vec<String>>());
    println!(
        "Part Numbers are {:#?}",
        part_numbers
            .iter()
            .map(|x| x.content.clone())
            .collect::<Vec<String>>()
    );
    println!("sum is {:#?}", part_numbers_sum);
}

pub fn solve_pt2() {
    let mut file = File::open("inputs/day_03_pt2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    // vector containing found numbers
    let mut numbers: Vec<Number> = Vec::new();
    let mut stars: Vec<(usize, usize)> = Vec::new();

    for (line_id, line) in lines.iter().enumerate() {
        let mut number: Option<Number> = None;

        for (char_id, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                // we found a number and now we look for its end and symbols
                if let Some(num) = number.as_mut() {
                    num.content.push(c);
                    num.columns.push(char_id);
                } else {
                    number = Some(Number {
                        line_id,
                        columns: vec![char_id],
                        content: String::from(c),
                    });
                }
            } else {
                if number.is_some() {
                    // the number is finished, we add it to the list of numbers
                    numbers.push(number.unwrap());
                    number = None;
                } else {
                    // nothing to do, we go on
                }

                if c == '*' {
                    // if we found a symbol, then we save its coordinates
                    stars.push((line_id, char_id));
                }
            }
        }
        if number.is_some() {
            numbers.push(number.unwrap());
        }
    }

    let mut gears_sum = 0;

    for star in stars {
        let candidates: Vec<&Number> = numbers
            .iter()
            .filter(|n| {
                ((n.line_id as i32 >= star.0 as i32 - 1) & (n.line_id <= star.0 + 1))
                    & (n.columns.contains(&((star.1 as i32 - 1).max(0) as usize))
                        | n.columns.contains(&star.1)
                        | n.columns.contains(&(star.1 + 1)))
            })
            .collect();

        /*println!("cadidates for {:?} are {:#?}", star, candidates.iter()
        .map(|x| x.content.clone())
        .collect::<Vec<String>>());*/
        if candidates.len() == 2 {
            gears_sum += candidates[0].content.parse::<i32>().unwrap()
                * candidates[1].content.parse::<i32>().unwrap();
        }
    }
    println!("gears sum {gears_sum}");
}
