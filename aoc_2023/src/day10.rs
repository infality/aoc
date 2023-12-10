#[derive(Debug)]
enum Dir {
    Left,
    Up,
    Right,
    Down,
}

fn get(field: &Vec<Vec<char>>, pos: (usize, usize)) -> char {
    field[pos.1][pos.0]
}

fn left(pos: (usize, usize)) -> (Dir, (usize, usize)) {
    (Dir::Right, (pos.0 - 1, pos.1))
}

fn up(pos: (usize, usize)) -> (Dir, (usize, usize)) {
    (Dir::Down, (pos.0, pos.1 - 1))
}

fn right(pos: (usize, usize)) -> (Dir, (usize, usize)) {
    (Dir::Left, (pos.0 + 1, pos.1))
}

fn down(pos: (usize, usize)) -> (Dir, (usize, usize)) {
    (Dir::Up, (pos.0, pos.1 + 1))
}

fn next_pos(last: &Dir, pos: (usize, usize), c: char) -> (Dir, (usize, usize)) {
    match (&last, c) {
        (Dir::Left, '-') => right(pos),
        (Dir::Left, 'J') => up(pos),
        (Dir::Left, '7') => down(pos),
        (Dir::Up, '|') => down(pos),
        (Dir::Up, 'J') => left(pos),
        (Dir::Up, 'L') => right(pos),
        (Dir::Right, '-') => left(pos),
        (Dir::Right, 'L') => up(pos),
        (Dir::Right, 'F') => down(pos),
        (Dir::Down, '|') => up(pos),
        (Dir::Down, '7') => left(pos),
        (Dir::Down, 'F') => right(pos),
        _ => unreachable!(),
    }
}

pub fn task1(input: &str) {
    let result;

    let field: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut pos = None;
    for (y, row) in field.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                pos = Some((x, y));
                break;
            }
        }
        if pos.is_some() {
            break;
        }
    }

    let mut pos = pos.unwrap();
    let mut last;
    if pos.0 > 0 && "-LF".contains(get(&field, left(pos).1)) {
        (last, pos) = left(pos);
    } else if pos.0 + 1 < field[0].len() && "-7J".contains(get(&field, right(pos).1)) {
        (last, pos) = right(pos);
    } else if pos.1 > 0 && "|7F".contains(get(&field, up(pos).1)) {
        (last, pos) = up(pos);
    } else if pos.1 + 1 < field.len() && "-JL".contains(get(&field, down(pos).1)) {
        (last, pos) = down(pos);
    } else {
        unreachable!();
    }

    let mut count = 0;
    loop {
        count += 1;
        let c = get(&field, pos);
        if c == 'S' {
            break;
        }
        (last, pos) = next_pos(&last, pos, c);
    }

    result = count / 2;

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut field: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut pos = None;
    for (y, row) in field.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                pos = Some((x, y));
                break;
            }
        }
        if pos.is_some() {
            break;
        }
    }

    let mut pos = pos.unwrap();
    let mut last;
    if pos.0 > 0 && "-LF".contains(get(&field, left(pos).1)) {
        (last, pos) = left(pos);
    } else if pos.0 + 1 < field[0].len() && "-7J".contains(get(&field, right(pos).1)) {
        (last, pos) = right(pos);
    } else if pos.1 > 0 && "|7F".contains(get(&field, up(pos).1)) {
        (last, pos) = up(pos);
    } else if pos.1 + 1 < field.len() && "-JL".contains(get(&field, down(pos).1)) {
        (last, pos) = down(pos);
    } else {
        unreachable!();
    }

    let mut history = Vec::new();
    loop {
        history.push(pos);
        let c = get(&field, pos);
        if c == 'S' {
            break;
        }
        (last, pos) = next_pos(&last, pos, c);
    }

    // easier than writing functionality to replace the S tile
    field[pos.1][pos.0] = 'F';

    for y in 0..field.len() {
        println!("row: {y}");
        for x in 0..field[0].len() {
            if history.contains(&(x, y)) {
                continue;
            }

            let mut walls = 0;
            let mut wall_dir = None;
            for xi in 0..x {
                if !history.contains(&(xi, y)) {
                    continue;
                }

                if wall_dir.is_some() {
                    match get(&field, (xi, y)) {
                        '7' | 'F' => {
                            if matches!(wall_dir.unwrap(), Dir::Up) {
                                walls += 1;
                            }
                            wall_dir = None;
                        }
                        'J' | 'L' => {
                            if matches!(wall_dir.unwrap(), Dir::Down) {
                                walls += 1;
                            }
                            wall_dir = None;
                        }
                        _ => (),
                    }
                    continue;
                }

                match get(&field, (xi, y)) {
                    '|' => walls += 1,
                    '7' | 'F' => wall_dir = Some(Dir::Down),
                    'J' | 'L' => wall_dir = Some(Dir::Up),
                    _ => (),
                }
            }
            println!("{walls}");
            if walls % 2 == 1 {
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}
