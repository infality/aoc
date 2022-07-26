use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut instructions = Vec::new();
    for line in input.lines() {
        let mut instruction = Vec::new();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            instruction.push(match c {
                'e' => Direction::East,
                'w' => Direction::West,
                's' => match chars.next().unwrap() {
                    'e' => Direction::SouthEast,
                    'w' => Direction::SouthWest,
                    _ => panic!(),
                },
                'n' => match chars.next().unwrap() {
                    'e' => Direction::NorthEast,
                    'w' => Direction::NorthWest,
                    _ => panic!(),
                },
                _ => panic!(),
            });
        }
        instructions.push(instruction);
    }

    let flipped = get_flipped_tiles(&instructions);
    result = flipped.len();

    println!("Result: {}", result);
}

fn get_flipped_tiles(instructions: &Vec<Vec<Direction>>) -> HashSet<(isize, isize)> {
    let mut flipped = HashSet::new();
    for instruction in instructions.iter() {
        let mut x = 0isize;
        let mut y = 0isize;
        for direction in instruction.iter() {
            match direction {
                Direction::East => x += 1,
                Direction::SouthEast => {
                    y += 1;
                    if y % 2 == 0 {
                        x += 1;
                    }
                }
                Direction::SouthWest => {
                    y += 1;
                    if y % 2 != 0 {
                        x -= 1;
                    }
                }
                Direction::West => x -= 1,
                Direction::NorthWest => {
                    y -= 1;
                    if y % 2 != 0 {
                        x -= 1;
                    }
                }
                Direction::NorthEast => {
                    y -= 1;
                    if y % 2 == 0 {
                        x += 1;
                    }
                }
            }
        }
        if !flipped.remove(&(x, y)) {
            flipped.insert((x, y));
        }
    }
    flipped
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut instructions = Vec::new();
    for line in input.lines() {
        let mut instruction = Vec::new();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            instruction.push(match c {
                'e' => Direction::East,
                'w' => Direction::West,
                's' => match chars.next().unwrap() {
                    'e' => Direction::SouthEast,
                    'w' => Direction::SouthWest,
                    _ => panic!(),
                },
                'n' => match chars.next().unwrap() {
                    'e' => Direction::NorthEast,
                    'w' => Direction::NorthWest,
                    _ => panic!(),
                },
                _ => panic!(),
            });
        }
        instructions.push(instruction);
    }

    let mut flipped = get_flipped_tiles(&instructions);

    for day in 0..100 {
        println!("{}: {}", day, flipped.len());

        /* for y in -6..6 {
            if y % 2 == 0 {
                print!("-");
            } else {
                print!("+");
            }
            for x in -6..6 {
                if flipped.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        if day == 5 {
            break;
        } */

        let mut new_flipped = flipped.clone();
        for (x, y) in flipped.iter() {
            let black = get_black_adjacent(*x, *y, &flipped);
            if black == 0 || black > 2 {
                new_flipped.remove(&(*x, *y));
            }
        }

        for (x, y) in flipped.iter() {
            for neighbor in get_neighbors(*x, *y) {
                check_white(neighbor.0, neighbor.1, &flipped, &mut new_flipped);
            }
        }

        flipped = new_flipped;
    }
    result = flipped.len();

    println!("Result: {}", result);
}

fn check_white(
    x: isize,
    y: isize,
    flipped: &HashSet<(isize, isize)>,
    new_flipped: &mut HashSet<(isize, isize)>,
) {
    if !flipped.contains(&(x, y)) && get_black_adjacent(x, y, flipped) == 2 {
        new_flipped.insert((x, y));
    }
}

fn get_black_adjacent(x: isize, y: isize, flipped: &HashSet<(isize, isize)>) -> usize {
    let mut result = 0;
    for neighbor in get_neighbors(x, y) {
        if flipped.contains(&neighbor) {
            result += 1;
        }
    }
    result
}

fn get_neighbors(x: isize, y: isize) -> Vec<(isize, isize)> {
    let mut result = Vec::new();
    result.push((x + 1, y));
    result.push((x - 1, y));
    result.push((x, y + 1));
    result.push((x, y - 1));
    if y % 2 == 0 {
        result.push((x - 1, y + 1));
        result.push((x - 1, y - 1));
    } else {
        result.push((x + 1, y + 1));
        result.push((x + 1, y - 1));
    }
    result
}
