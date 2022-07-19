#[derive(PartialEq, Debug, Clone)]
enum Cucumber {
    East,
    South,
}

pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut state = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '>' => Some(Cucumber::East),
                'v' => Some(Cucumber::South),
                '.' => None,
                _ => panic!(),
            });
        }
        state.push(row);
    }

    let mut counter = 0;
    let rows = state.len();
    let cols = state[0].len();
    loop {
        let mut new_state = state.clone();
        let mut changed = false;

        for row in 0..rows {
            for col in 0..cols {
                if state[row][col] == Some(Cucumber::East) && state[row][(col + 1) % cols].is_none()
                {
                    new_state[row][(col + 1) % cols] = Some(Cucumber::East);
                    new_state[row][col] = None;
                    changed = true;
                }
            }
        }

        if changed {
            state = new_state.clone();
        }

        for row in 0..rows {
            for col in 0..cols {
                if state[row][col] == Some(Cucumber::South)
                    && state[(row + 1) % rows][col].is_none()
                {
                    new_state[(row + 1) % rows][col] = Some(Cucumber::South);
                    new_state[row][col] = None;
                    changed = true;
                }
            }
        }

        counter += 1;
        if !changed {
            result = counter;
            break;
        }
        state = new_state;

        if counter % 100 == 0 {
            println!("{}", counter);
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;
    println!("Result: {}", result);
}
