use std::collections::HashMap;

pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut orbits = HashMap::new();
    for line in input.lines() {
        let parts = line.split_once(')').unwrap();
        assert!(orbits.insert(parts.1, parts.0).is_none());
    }

    for o1 in orbits.keys() {
        let mut current = o1;
        while let Some(o2) = orbits.get(current) {
            current = o2;
            result += 1;
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut orbits = HashMap::new();
    for line in input.lines() {
        let parts = line.split_once(')').unwrap();
        assert!(orbits.insert(parts.1, parts.0).is_none());
    }

    let mut list = Vec::new();
    let mut current = "YOU";
    while let Some(o) = orbits.get(current) {
        list.push(o);
        current = o;
    }

    let mut common = "";
    current = "SAN";
    while let Some(o) = orbits.get(current) {
        result += 1;
        if list.contains(&o) {
            common = o;
            break;
        }
        current = o;
    }
    println!("Common: {}", common);

    current = "YOU";
    while let Some(o) = orbits.get(current) {
        if *o == common {
            break;
        }
        result += 1;
        current = o;
    }

    println!("Result: {}", result - 1);
}
