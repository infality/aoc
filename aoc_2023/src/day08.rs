use std::collections::{HashMap, HashSet};

pub fn task1(input: &str) {
    let mut result = 0usize;

    let parts = input.split_once("\n\n").unwrap();
    let instructions = parts.0;

    let mut map = HashMap::new();
    for (i, line) in parts.1.lines().enumerate() {
        let name = line.split_once(" = ").unwrap().0;
        map.insert(name, i);
    }
    let start = *map.get("AAA").unwrap();
    let end = *map.get("ZZZ").unwrap();

    let mut nodes = Vec::new();
    for line in parts.1.lines() {
        let ways = line.split_once(" = ").unwrap().1.split_once(", ").unwrap();
        let left = map.get(&ways.0[1..]).unwrap();
        let right = map.get(&ways.1[..3]).unwrap();
        nodes.push([*left, *right]);
    }

    let mut current = start;
    loop {
        let direction = match instructions
            .chars()
            .nth(result % instructions.len())
            .unwrap()
        {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        };
        result += 1;
        current = nodes[current][direction];
        if current == end {
            break;
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let parts = input.split_once("\n\n").unwrap();
    let instructions = parts.0;

    let mut map = HashMap::new();
    let mut current_nodes = Vec::new();
    let mut ends = HashSet::new();
    for (i, line) in parts.1.lines().enumerate() {
        let name = line.split_once(" = ").unwrap().0;
        map.insert(name, i);
        if name.contains('A') {
            current_nodes.push(i);
        }
        if name.contains('Z') {
            ends.insert(i);
        }
    }

    let mut nodes = Vec::new();
    for line in parts.1.lines() {
        let ways = line.split_once(" = ").unwrap().1.split_once(", ").unwrap();
        let left = map.get(&ways.0[1..]).unwrap();
        let right = map.get(&ways.1[..3]).unwrap();
        nodes.push([*left, *right]);
    }

    let mut cycles = vec![None; current_nodes.len()];
    loop {
        let direction = match instructions
            .chars()
            .nth(result % instructions.len())
            .unwrap()
        {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        };
        result += 1;
        for (i, current) in current_nodes.iter_mut().enumerate() {
            *current = nodes[*current][direction];
            if cycles[i].is_none() && ends.contains(current) {
                cycles[i] = Some(result);
            }
        }
        if cycles.iter().all(|x| x.is_some()) {
            break;
        }
    }

    result = cycles[0].unwrap();
    for cycle in cycles.iter().skip(1) {
        result = lcm(result, cycle.unwrap());
    }

    println!("Result: {}", result);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}
