pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut data = Vec::new();
    for line in input.split('\n') {
        let mut row = Vec::new();
        for entry in line.chars() {
            row.push((entry.to_string().parse::<usize>().unwrap(), false));
        }
        data.push(row);
    }

    for _ in 0..100 {
        for row in data.iter_mut() {
            for entry in row.iter_mut() {
                entry.0 += 1;
            }
        }

        let mut changes = true;
        while changes {
            changes = false;

            for i in 0..data.len() {
                for j in 0..data[0].len() {
                    if data[i][j].0 <= 9 || data[i][j].1 {
                        continue;
                    }
                    changes = true;
                    data[i][j].1 = true;

                    let x1 = if i > 0 { i - 1 } else { 0 };
                    let x2 = if i < data.len() - 1 {
                        i + 1
                    } else {
                        data.len() - 1
                    };
                    let y1 = if j > 0 { j - 1 } else { 0 };
                    let y2 = if j < data[0].len() - 1 {
                        j + 1
                    } else {
                        data[0].len() - 1
                    };

                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            if x == i && y == j {
                                continue;
                            }
                            data[x][y].0 += 1;
                        }
                    }
                }
            }
        }

        for row in data.iter_mut() {
            for entry in row.iter_mut() {
                if entry.1 {
                    result += 1;
                    entry.0 = 0;
                    entry.1 = false;
                }
            }
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut data = Vec::new();
    for line in input.split('\n') {
        let mut row = Vec::new();
        for entry in line.chars() {
            row.push((entry.to_string().parse::<usize>().unwrap(), false));
        }
        data.push(row);
    }

    for step in 0..10000 {
        for row in data.iter_mut() {
            for entry in row.iter_mut() {
                entry.0 += 1;
            }
        }

        let mut changes = true;
        while changes {
            changes = false;

            for i in 0..data.len() {
                for j in 0..data[0].len() {
                    if data[i][j].0 <= 9 || data[i][j].1 {
                        continue;
                    }
                    changes = true;
                    data[i][j].1 = true;

                    let x1 = if i > 0 { i - 1 } else { 0 };
                    let x2 = if i < data.len() - 1 {
                        i + 1
                    } else {
                        data.len() - 1
                    };
                    let y1 = if j > 0 { j - 1 } else { 0 };
                    let y2 = if j < data[0].len() - 1 {
                        j + 1
                    } else {
                        data[0].len() - 1
                    };

                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            if x == i && y == j {
                                continue;
                            }
                            data[x][y].0 += 1;
                        }
                    }
                }
            }
        }

        if data.iter().all(|x| x.iter().all(|y| y.1)) {
            result = step + 1;
            break;
        }

        for row in data.iter_mut() {
            for entry in row.iter_mut() {
                if entry.1 {
                    entry.0 = 0;
                    entry.1 = false;
                }
            }
        }
    }

    println!("Result: {}", result);
}
