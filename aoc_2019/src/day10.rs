pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut data = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                data.push((x, y));
            }
        }
    }

    let mut pos = (0, 0);
    for asteroid in data.iter() {
        let mut slopes = Vec::new();
        for other in data.iter() {
            if other == asteroid {
                continue;
            }
            let slope = (other.1 as f64 - asteroid.1 as f64) / (other.0 as f64 - asteroid.0 as f64);
            let is_right = other.0 > asteroid.0;
            if !slopes.contains(&(is_right, slope)) {
                slopes.push((is_right, slope));
            }
        }
        if slopes.len() > result {
            result = slopes.len();
            pos = *asteroid;
        }
    }

    println!("Coords: {},{}", pos.0, pos.1);
    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let result;

    let mut data = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                data.push((x, y));
            }
        }
    }

    let mut pos = (0, 0);
    let mut visible = 0;
    for asteroid in data.iter() {
        let mut slopes = Vec::new();
        for other in data.iter() {
            if other == asteroid {
                continue;
            }
            let slope = (other.1 as f64 - asteroid.1 as f64) / (other.0 as f64 - asteroid.0 as f64);
            let is_right = other.0 >= asteroid.0;
            if !slopes.contains(&(is_right, slope)) {
                slopes.push((is_right, slope));
            }
        }
        if slopes.len() > visible {
            visible = slopes.len();
            pos = *asteroid;
        }
    }

    let mut slopes = Vec::new();
    for other in data.iter() {
        if *other == pos {
            continue;
        }
        let slope = (other.1 as f64 - pos.1 as f64) / (other.0 as f64 - pos.0 as f64);
        let is_right = other.0 >= pos.0;
        slopes.push((is_right, slope, other));
    }

    let mut slope = f64::NEG_INFINITY;
    let mut is_right = true;
    let mut i = 0;
    loop {
        let mut candidate: Option<&(bool, f64, &(usize, usize))> = None;
        let mut index = 0;
        for (j, current) in slopes.iter().enumerate() {
            if current.0 != is_right {
                continue;
            }
            if is_right {
                if (current.1 > slope
                    || (slope == f64::NEG_INFINITY && current.1 == f64::NEG_INFINITY))
                    && (candidate.is_none() || candidate.unwrap().1 >= current.1)
                {
                    if candidate.is_some()
                        && candidate.unwrap().1 == current.1
                        && pos.0.abs_diff(current.2 .0) > pos.0.abs_diff(candidate.unwrap().2 .0)
                    {
                        continue;
                    }
                    candidate = Some(current);
                    index = j;
                }
            } else {
                if current.1 > slope && (candidate.is_none() || candidate.unwrap().1 >= current.1) {
                    if candidate.is_some()
                        && candidate.unwrap().1 == current.1
                        && pos.0.abs_diff(current.2 .0) > pos.0.abs_diff(candidate.unwrap().2 .0)
                    {
                        continue;
                    }
                    candidate = Some(current);
                    index = j;
                }
            }
        }

        if candidate.is_none() {
            slope = f64::NEG_INFINITY;
            is_right = !is_right;
            continue;
        }

        i += 1;
        if i == 200 {
            result = candidate.unwrap().2 .0 * 100 + candidate.unwrap().2 .1;
            break;
        }

        if is_right && candidate.unwrap().1 == f64::NEG_INFINITY {
            slope = f64::MIN;
        } else {
            slope = candidate.unwrap().1;
        }
        slopes.swap_remove(index);
    }

    println!("Result: {}", result);
}
