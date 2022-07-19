use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum CaveType {
    Small(String),
    Big(String),
}

pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut map = HashMap::<CaveType, Vec<CaveType>>::new();
    for line in input.lines() {
        let parts = line.split_once('-').unwrap();
        let left = if parts.0.to_lowercase() == parts.0 {
            CaveType::Small(parts.0.to_string())
        } else {
            CaveType::Big(parts.0.to_string())
        };
        let right = if parts.1.to_lowercase() == parts.1 {
            CaveType::Small(parts.1.to_string())
        } else {
            CaveType::Big(parts.1.to_string())
        };
        map.entry(left.clone()).or_default().push(right.clone());
        map.entry(right).or_default().push(left);
    }

    let start = CaveType::Small("start".to_string());
    result = explore(vec![start.clone()], start, &map);

    println!("Result: {}", result);
}

fn explore(
    visited_small_caves: Vec<CaveType>,
    current_cave: CaveType,
    map: &HashMap<CaveType, Vec<CaveType>>,
) -> usize {
    let mut result = 0;
    for dest in map.get(&current_cave).unwrap().iter() {
        if visited_small_caves.contains(&dest) {
            continue;
        }
        if *dest == CaveType::Small("end".to_string()) {
            result += 1;
            continue;
        }

        let mut visited = visited_small_caves.clone();
        match dest {
            CaveType::Small(_) => visited.push(dest.clone()),
            _ => (),
        }
        result += explore(visited, dest.clone(), map)
    }
    result
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut map = HashMap::<CaveType, Vec<CaveType>>::new();
    for line in input.lines() {
        let parts = line.split_once('-').unwrap();
        let left = if parts.0.to_lowercase() == parts.0 {
            CaveType::Small(parts.0.to_string())
        } else {
            CaveType::Big(parts.0.to_string())
        };
        let right = if parts.1.to_lowercase() == parts.1 {
            CaveType::Small(parts.1.to_string())
        } else {
            CaveType::Big(parts.1.to_string())
        };
        map.entry(left.clone()).or_default().push(right.clone());
        map.entry(right).or_default().push(left);
    }

    let start = CaveType::Small("start".to_string());
    result = explore2(vec![start.clone()], start.clone(), &map, vec![start], false);

    println!("Result: {}", result);
}

fn explore2(
    visited_small_caves: Vec<CaveType>,
    current_cave: CaveType,
    map: &HashMap<CaveType, Vec<CaveType>>,
    path: Vec<CaveType>,
    used_double: bool,
) -> usize {
    let mut result = 0;
    for dest in map.get(&current_cave).unwrap().iter() {
        if *dest == CaveType::Small("end".to_string()) {
            result += 1;
            continue;
        }
        let mut path = path.clone();
        path.push(dest.clone());
        if visited_small_caves.contains(&dest) {
            if !used_double && *dest != CaveType::Small("start".to_string()) {
                result += explore2(visited_small_caves.clone(), dest.clone(), map, path, true);
            }
            continue;
        }

        let mut visited = visited_small_caves.clone();
        match dest {
            CaveType::Small(_) => visited.push(dest.clone()),
            _ => (),
        }
        result += explore2(visited, dest.clone(), map, path, used_double);
    }
    result
}
