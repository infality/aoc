pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut numbers = Vec::new();
    for line in input.lines() {
        numbers.push(line.chars().collect::<Vec<char>>());
    }

    let number = calc_sum(&numbers);
    result = get_result(&number);

    println!("Result: {}", result);
}

fn calc_sum(numbers: &Vec<Vec<char>>) -> Vec<char> {
    let mut number = numbers.first().unwrap().clone();
    for num_index in 1..numbers.len() {
        number.insert(0, '[');
        number.push(',');
        number.extend(numbers[num_index].clone());
        number.push(']');

        let mut had_changes = true;
        while had_changes {
            had_changes = false;

            let mut pos = 0;
            let mut depth = 0;
            let mut last_literal_pos = None;
            let mut split_pos = None;
            while depth <= 4 && pos < number.len() {
                if number[pos] == '[' {
                    depth += 1;
                    pos += 1;
                } else if number[pos] == ']' {
                    depth -= 1;
                    pos += 1;
                } else if number[pos] == ',' {
                    pos += 1;
                } else {
                    last_literal_pos = Some(pos);
                    let (value, literal_len) = get_value(&number[pos..]);

                    if value > 9 && split_pos.is_none() {
                        split_pos = Some(pos);
                    }

                    pos += literal_len;
                }
            }

            // Explode
            if depth > 4 {
                let (left_value, left_value_len) = get_value(&number[pos..]);
                let right_value = get_value(&number[pos + left_value_len + 1..]).0;
                pos -= 1;
                number[pos] = '0';
                let close_bracket = pos + number.iter().skip(pos).position(|x| *x == ']').unwrap();
                for i in ((pos + 1)..=close_bracket).rev() {
                    number.remove(i);
                }

                let next_literal_offset = number
                    .iter()
                    .skip(pos + 2)
                    .position(|x| *x != '[' && *x != ']' && *x != ',');
                if next_literal_offset.is_some() {
                    let next_literal_pos = pos + 2 + next_literal_offset.unwrap();
                    let (mut literal, literal_len) = get_value(&number[next_literal_pos..]);
                    literal += right_value;
                    for i in (0..literal_len).rev() {
                        number.remove(next_literal_pos + i);
                    }
                    let literal = literal.to_string();
                    for (i, c) in literal.char_indices() {
                        number.insert(next_literal_pos + i, c);
                    }
                }

                if let Some(last_literal_pos) = last_literal_pos {
                    let (mut literal, literal_len) = get_value(&number[last_literal_pos..]);
                    literal += left_value;
                    for i in (0..literal_len).rev() {
                        number.remove(last_literal_pos + i);
                    }
                    let literal = literal.to_string();
                    for (i, c) in literal.char_indices() {
                        number.insert(last_literal_pos + i, c);
                    }
                }
                had_changes = true;
            }

            // Split
            if !had_changes && split_pos.is_some() {
                pos = split_pos.unwrap();
                let (value, literal_len) = get_value(&number[pos..]);

                for i in (0..literal_len).rev() {
                    number.remove(pos + i);
                }
                number.insert(pos, '[');
                pos += 1;

                let left_value = (value / 2).to_string();
                for c in left_value.chars() {
                    number.insert(pos, c);
                    pos += 1;
                }

                number.insert(pos, ',');
                pos += 1;

                let right_value = ((value + 1) / 2).to_string();
                for c in right_value.chars() {
                    number.insert(pos, c);
                    pos += 1;
                }

                number.insert(pos, ']');
                had_changes = true;
            }
        }
    }
    number
}

fn get_value(data: &[char]) -> (usize, usize) {
    let literal_len = data.iter().position(|x| *x == ',' || *x == ']').unwrap();
    (
        data.iter()
            .take(literal_len)
            .collect::<String>()
            .parse::<usize>()
            .unwrap(),
        literal_len,
    )
}

fn get_result(number: &[char]) -> usize {
    let mut depth_values = vec![None; 4];
    let mut depth = 0;
    let mut pos = 1;
    while pos < number.len() - 1 {
        let c = number[pos];
        if c == '[' {
            depth += 1;
            pos += 1;
        } else if c == ']' {
            depth -= 1;
            if depth_values[depth].is_none() {
                depth_values[depth] = Some(3 * depth_values[depth + 1].unwrap());
            } else {
                depth_values[depth] =
                    Some(depth_values[depth].unwrap() + 2 * depth_values[depth + 1].unwrap());
            }
            depth_values[depth + 1] = None;
            pos += 1;
        } else if c == ',' {
            pos += 1;
        } else {
            let (value, literal_len) = get_value(&number[pos..]);
            if depth_values[depth].is_none() {
                depth_values[depth] = Some(3 * value);
            } else {
                depth_values[depth] = Some(depth_values[depth].unwrap() + 2 * value);
            }
            pos += literal_len;
        }
    }
    depth_values[0].unwrap()
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut numbers = Vec::new();
    for line in input.lines() {
        numbers.push(line.chars().collect::<Vec<char>>());
    }

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let nums = vec![numbers[i].clone(), numbers[j].clone()];
            let number = calc_sum(&nums);
            result = result.max(get_result(&number));
        }
    }

    println!("Result: {}", result);
}
