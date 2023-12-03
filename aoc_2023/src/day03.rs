fn sub(value: usize) -> usize {
    if value == 0 {
        0
    } else {
        value - 1
    }
}

pub fn task1(input: &str) {
    let mut result = 0usize;

    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines[0].len();
    let height = lines.len();

    for i in 0..height {
        let line = lines[i];

        let mut start = None;
        for j in 0..width {
            let char = line.chars().nth(j).unwrap();
            let mut end = None;

            if start.is_some() {
                if !char.is_digit(10) {
                    end = Some(j);
                } else if j == width - 1 {
                    end = Some(width);
                }
            } else {
                if char == '.' {
                    continue;
                }
                if char.is_digit(10) {
                    start = Some(j);
                }
            }

            if end.is_some() {
                let mut valid = false;
                for i2 in sub(i)..height.min(i + 2) {
                    for j2 in sub(start.unwrap())..width.min(end.unwrap() + 1) {
                        let char = lines[i2].chars().nth(j2).unwrap();
                        if char != '.' && !char.is_digit(10) {
                            valid = true;
                            break;
                        }
                    }
                    if valid {
                        break;
                    }
                }

                if valid {
                    result += line[start.unwrap()..end.unwrap()].parse::<usize>().unwrap();
                }
                start = None;
            }
        }
    }

    println!("Result: {}", result);
}

fn get_number(line: &str, pos: usize) -> Option<usize> {
    if !line.chars().nth(pos).unwrap().is_digit(10) {
        return None;
    }
    let mut start = pos;
    let mut last = pos;
    for i in (0..pos).rev() {
        if line.chars().nth(i).unwrap().is_digit(10) {
            start = i;
        } else {
            break;
        }
    }
    for i in (pos + 1)..line.len() {
        if line.chars().nth(i).unwrap().is_digit(10) {
            last = i;
        } else {
            break;
        }
    }
    Some(line[start..=last].parse().unwrap())
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines[0].len();
    let height = lines.len();

    for i in 0..height {
        for j in 0..width {
            if lines[i].chars().nth(j).unwrap() == '*' {
                let mut value = 1;
                let mut value_count = 0;

                if j > 0 {
                    if let Some(v) = get_number(lines[i], j - 1) {
                        value *= v;
                        value_count += 1;
                    }
                }
                if j < width - 1 {
                    if let Some(v) = get_number(lines[i], j + 1) {
                        value *= v;
                        value_count += 1;
                    }
                }

                if i > 0 {
                    if let Some(v) = get_number(lines[i - 1], j) {
                        value *= v;
                        value_count += 1;
                    } else {
                        if j > 0 {
                            if let Some(v) = get_number(lines[i - 1], j - 1) {
                                value *= v;
                                value_count += 1;
                            }
                        }
                        if j < width - 1 {
                            if let Some(v) = get_number(lines[i - 1], j + 1) {
                                value *= v;
                                value_count += 1;
                            }
                        }
                    }
                }

                if i < height - 1 {
                    if let Some(v) = get_number(lines[i + 1], j) {
                        value *= v;
                        value_count += 1;
                    } else {
                        if j > 0 {
                            if let Some(v) = get_number(lines[i + 1], j - 1) {
                                value *= v;
                                value_count += 1;
                            }
                        }
                        if j < width - 1 {
                            if let Some(v) = get_number(lines[i + 1], j + 1) {
                                value *= v;
                                value_count += 1;
                            }
                        }
                    }
                }

                if value_count == 2 {
                    result += value;
                }
            }
        }
    }

    println!("Result: {}", result);
}
