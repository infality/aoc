pub fn task1(input: &str) {
    let mut result = 0usize;

    let parts = input.split_once("\n\n").unwrap();
    let data = parts.0;

    let border = 5;
    let width = parts.1.lines().next().unwrap().len() + 2 * border;
    let height = parts.1.lines().count() + 2 * border;
    let mut image = Vec::new();
    for _ in 0..border {
        image.push(vec![false; width]);
    }
    for line in parts.1.lines() {
        let mut row = vec![false; border];
        for c in line.chars() {
            row.push(match c {
                '#' => true,
                '.' => false,
                _ => panic!(),
            });
        }
        for _ in 0..border {
            row.push(false);
        }
        image.push(row);
    }
    for _ in 0..border {
        image.push(vec![false; width]);
    }

    for _ in 0..2 {
        let mut new_image = image.clone();
        for row in 0..height {
            for col in 0..width {
                let mut binary = String::new();
                let left = if col == 0 { 0 } else { col - 1 };
                let top = if row == 0 { 0 } else { row - 1 };
                let right = (col + 1).min(width - 1);
                let bottom = (row + 1).min(height - 1);
                binary.push(if image[top][left] { '1' } else { '0' });
                binary.push(if image[top][col] { '1' } else { '0' });
                binary.push(if image[top][right] { '1' } else { '0' });
                binary.push(if image[row][left] { '1' } else { '0' });
                binary.push(if image[row][col] { '1' } else { '0' });
                binary.push(if image[row][right] { '1' } else { '0' });
                binary.push(if image[bottom][left] { '1' } else { '0' });
                binary.push(if image[bottom][col] { '1' } else { '0' });
                binary.push(if image[bottom][right] { '1' } else { '0' });
                new_image[row][col] = data
                    .chars()
                    .nth(usize::from_str_radix(&binary, 2).unwrap())
                    .unwrap()
                    == '#';
            }
        }
        image = new_image;

        for row in image.iter() {
            for value in row.iter() {
                print!("{}", if *value { '#' } else { '.' });
            }
            println!();
        }
    }

    for row in image.iter() {
        for value in row.iter() {
            if *value {
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let parts = input.split_once("\n\n").unwrap();
    let data = parts.0;

    let border = 60;
    let width = parts.1.lines().next().unwrap().len() + 2 * border;
    let height = parts.1.lines().count() + 2 * border;
    let mut image = Vec::new();
    for _ in 0..border {
        image.push(vec![false; width]);
    }
    for line in parts.1.lines() {
        let mut row = vec![false; border];
        for c in line.chars() {
            row.push(match c {
                '#' => true,
                '.' => false,
                _ => panic!(),
            });
        }
        for _ in 0..border {
            row.push(false);
        }
        image.push(row);
    }
    for _ in 0..border {
        image.push(vec![false; width]);
    }

    for _ in 0..50 {
        let mut new_image = image.clone();
        for row in 0..height {
            for col in 0..width {
                let mut binary = String::new();
                let left = if col == 0 { 0 } else { col - 1 };
                let top = if row == 0 { 0 } else { row - 1 };
                let right = (col + 1).min(width - 1);
                let bottom = (row + 1).min(height - 1);
                binary.push(if image[top][left] { '1' } else { '0' });
                binary.push(if image[top][col] { '1' } else { '0' });
                binary.push(if image[top][right] { '1' } else { '0' });
                binary.push(if image[row][left] { '1' } else { '0' });
                binary.push(if image[row][col] { '1' } else { '0' });
                binary.push(if image[row][right] { '1' } else { '0' });
                binary.push(if image[bottom][left] { '1' } else { '0' });
                binary.push(if image[bottom][col] { '1' } else { '0' });
                binary.push(if image[bottom][right] { '1' } else { '0' });
                new_image[row][col] = data
                    .chars()
                    .nth(usize::from_str_radix(&binary, 2).unwrap())
                    .unwrap()
                    == '#';
            }
        }
        image = new_image;

        /* for row in image.iter() {
            for value in row.iter() {
                print!("{}", if *value { '#' } else { '.' });
            }
            println!();
        } */
    }

    for row in image.iter() {
        for value in row.iter() {
            if *value {
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}
