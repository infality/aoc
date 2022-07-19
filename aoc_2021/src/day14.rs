use std::collections::HashMap;

pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut data = HashMap::new();
    let parts = input.split_once("\n\n").unwrap();
    for line in parts.1.split('\n') {
        let rule = line.split_once(" -> ").unwrap();
        data.insert(rule.0.to_string(), rule.1.chars().nth(0).unwrap());
    }

    let mut polymer = parts.0.to_string();
    for step in 0..10 {
        let mut new_polymer = String::from(polymer.chars().nth(0).unwrap());
        for (i, c2) in polymer.char_indices().skip(1) {
            let c1 = polymer.chars().nth(i - 1).unwrap();
            new_polymer.push(data.get(&format!("{}{}", c1, c2)).unwrap().clone());
            new_polymer.push(c2);
        }
        polymer = new_polymer.clone();
        println!(
            "Step {}, len {}: {}",
            step + 1,
            polymer.chars().count(),
            polymer
        );
    }

    let counts = polymer.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    result = *counts.iter().max_by_key(|x| x.1).unwrap().1
        - *counts.iter().min_by_key(|x| x.1).unwrap().1;

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut rules = HashMap::new();
    let parts = input.split_once("\n\n").unwrap();
    for line in parts.1.split('\n') {
        let rule = line.split_once(" -> ").unwrap();
        rules.insert(rule.0.to_string(), rule.1.chars().nth(0).unwrap());
    }

    let mut pairs = HashMap::new();
    for (i, c2) in parts.0.char_indices().skip(1) {
        let c1 = parts.0.chars().nth(i - 1).unwrap();
        *pairs.entry(format!("{}{}", c1, c2)).or_insert_with(|| 0) += 1;
    }

    println!("{:?}", pairs);
    for step in 0..40 {
        let before = pairs.clone();
        pairs.clear();
        for (pair, count) in before.iter() {
            let rule_result = rules.get(pair).unwrap();
            let pair1 = format!("{}{}", pair.chars().nth(0).unwrap(), rule_result);
            let pair2 = format!("{}{}", rule_result, pair.chars().nth(1).unwrap());
            pairs
                .entry(pair1)
                .and_modify(|x| *x += count)
                .or_insert(*count);
            pairs
                .entry(pair2)
                .and_modify(|x| *x += count)
                .or_insert(*count);
        }
        println!("{:?}", pairs);
    }

    let mut counts = pairs.iter().fold(HashMap::new(), |mut map, c| {
        *map.entry(c.0.chars().nth(0).unwrap()).or_insert(0) += c.1;
        map
    });
    *counts.entry(parts.0.chars().last().unwrap()).or_insert(0) += 1;

    result = *counts.iter().max_by_key(|x| x.1).unwrap().1
        - *counts.iter().min_by_key(|x| x.1).unwrap().1;

    println!("Result: {}", result);
}
