use std::ops::Range;

pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut steps = Vec::new();
    for line in input.lines() {
        let parts = line.split_once(' ').unwrap();
        let mut step = [0usize..0, 0..0, 0..0];
        for (i, axis) in parts.1.splitn(3, ',').enumerate() {
            let values = axis[2..].split_once("..").unwrap();
            step[i] = ((values.0.parse::<isize>().unwrap() + 50).clamp(0, 101)) as usize
                ..((values.1.parse::<isize>().unwrap() + 51).clamp(0, 101)) as usize;
        }
        steps.push((parts.0 == "on", step));
    }

    let mut cubes = [[[false; 101]; 101]; 101];

    for step in steps.iter() {
        for x in step.1[0].clone() {
            for y in step.1[1].clone() {
                for z in step.1[2].clone() {
                    cubes[x][y][z] = step.0;
                }
            }
        }
    }

    for x in cubes.iter() {
        for y in x.iter() {
            for value in y.iter() {
                if *value {
                    result += 1;
                }
            }
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0i128;

    let mut steps = Vec::new();
    for line in input.lines() {
        let parts = line.split_once(' ').unwrap();
        let mut step = [(0, 0); 3];
        for (i, axis) in parts.1.splitn(3, ',').enumerate() {
            let values = axis[2..].split_once("..").unwrap();
            step[i] = (
                values.0.parse::<isize>().unwrap(),
                values.1.parse::<isize>().unwrap() + 1,
            );
        }
        steps.push((parts.0 == "on", step));
    }

    let mut new_steps = vec![steps[0].clone()];
    for step in steps.iter().skip(1) {
        let mut additional_steps = Vec::new();
        for new_step in new_steps.iter() {
            if let Some(intersection) = intersection(step.1, new_step.1) {
                additional_steps.push((!new_step.0, intersection));
            }
        }
        if step.0 {
            new_steps.push(step.clone());
        }
        new_steps.extend(additional_steps);
    }

    for step in new_steps.iter() {
        let mut amount = 1;
        for axis in step.1.iter() {
            amount *= axis.1 - axis.0;
        }
        if step.0 {
            result += amount as i128;
        } else {
            result -= amount as i128;
        }
    }

    println!("Result: {}", result);
}

fn intersection(
    cube1: [(isize, isize); 3],
    cube2: [(isize, isize); 3],
) -> Option<[(isize, isize); 3]> {
    let x = range_intersection(&cube1[0], &cube2[0]);
    let y = range_intersection(&cube1[1], &cube2[1]);
    let z = range_intersection(&cube1[2], &cube2[2]);
    if x.is_none() || y.is_none() || z.is_none() {
        return None;
    }
    Some([x.unwrap(), y.unwrap(), z.unwrap()])
}

fn range_intersection(range1: &(isize, isize), range2: &(isize, isize)) -> Option<(isize, isize)> {
    let min1 = range1.0;
    let min2 = range2.0;
    let max1 = range1.1;
    let max2 = range2.1;
    if min1 >= max2 || max1 <= min2 {
        return None;
    }
    let min = min1.max(min2);
    let max = max1.min(max2);
    if min == max {
        return None;
    }
    Some((min, max))
}
