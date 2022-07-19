pub fn task1(input: &str) {
    let mut result = 0isize;

    let mut input_start = input.find(|x| x == '=').unwrap() + 1;
    let mut input_len = input[input_start..].find(|x| x == '.').unwrap();
    let x1 = input[input_start..(input_start + input_len)]
        .parse::<isize>()
        .unwrap();
    input_start = input_start + input_len + 2;
    input_len = input[input_start..].find(|x| x == ',').unwrap();
    let x2 = input[input_start..(input_start + input_len)]
        .parse::<isize>()
        .unwrap();

    input_start = input_start + input_len + 4;
    input_len = input[input_start..].find(|x| x == '.').unwrap();
    let y1 = input[input_start..(input_start + input_len)]
        .parse::<isize>()
        .unwrap();
    input_start = input_start + input_len + 2;
    let y2 = input[input_start..].parse::<isize>().unwrap();

    let mut max_height = 0;
    for x in 1..=x2 {
        for y in y2..1000 {
            let mut x_pos = 0;
            let mut y_pos = 0;
            let mut x_vel = x;
            let mut y_vel = y;
            let mut y_max = 0;
            let mut hit = false;
            loop {
                x_pos += x_vel;
                y_pos += y_vel;
                y_max = y_max.max(y_pos);

                if x_pos > x2 || y_pos < y1 {
                    break;
                }
                if x1 <= x_pos && x_pos <= x2 && y1 <= y_pos && y_pos <= y2 {
                    hit = true;
                    break;
                }

                if x_vel > 0 {
                    x_vel -= 1;
                }
                y_vel -= 1;
            }
            if hit {
                max_height = max_height.max(y_max);
            }
        }
    }
    result = max_height;

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut input_start = input.find(|x| x == '=').unwrap() + 1;
    let mut input_len = input[input_start..].find(|x| x == '.').unwrap();
    let x1 = input[input_start..(input_start + input_len)]
        .parse::<isize>()
        .unwrap();
    input_start = input_start + input_len + 2;
    input_len = input[input_start..].find(|x| x == ',').unwrap();
    let x2 = input[input_start..(input_start + input_len)]
        .parse::<isize>()
        .unwrap();

    input_start = input_start + input_len + 4;
    input_len = input[input_start..].find(|x| x == '.').unwrap();
    let y1 = input[input_start..(input_start + input_len)]
        .parse::<isize>()
        .unwrap();
    input_start = input_start + input_len + 2;
    let y2 = input[input_start..].parse::<isize>().unwrap();

    let mut hits = 0;
    for x in 1..=x2 {
        for y in y1..1000 {
            let mut x_pos = 0;
            let mut y_pos = 0;
            let mut x_vel = x;
            let mut y_vel = y;
            let mut hit = false;
            loop {
                x_pos += x_vel;
                y_pos += y_vel;

                if x_pos > x2 || y_pos < y1 {
                    break;
                }
                if x1 <= x_pos && x_pos <= x2 && y1 <= y_pos && y_pos <= y2 {
                    hit = true;
                    break;
                }

                if x_vel > 0 {
                    x_vel -= 1;
                }
                y_vel -= 1;
            }
            if hit {
                hits += 1;
            }
        }
    }
    result = hits;

    println!("Result: {}", result);
}
