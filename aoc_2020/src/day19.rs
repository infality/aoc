use std::collections::HashMap;

pub fn task1(input: &str) {
    let mut result = 0usize;
    println!("Result: {}", result);
}

#[derive(Debug, Clone)]
enum Entry {
    Literal(char),
    Rule(usize),
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut rules: HashMap<usize, Vec<Vec<Entry>>> = HashMap::new();
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let test_messages = parts[1].lines().collect::<Vec<&str>>();
    for line in parts[0].lines() {
        let mut rule = Vec::new();
        let line_parts = line.split_once(": ").unwrap();
        for rule_part in line_parts.1.split(" | ") {
            let mut rule_parts = Vec::new();
            for entry in rule_part.split(' ') {
                rule_parts.push(if entry.chars().any(|x| x == '"') {
                    Entry::Literal(entry.chars().skip(1).next().unwrap())
                } else {
                    Entry::Rule(entry.parse().unwrap())
                });
            }
            rule.push(rule_parts);
        }
        rules.insert(line_parts.0.parse().unwrap(), rule);
    }

    let mut sub_results: HashMap<usize, Vec<String>> = HashMap::new();
    for start in [31, 42] {
        sub_results.insert(start, get_messages(start, &rules));
    }
    println!("{:?}", sub_results);

    let min_sub_result_len = sub_results
        .iter()
        .map(|x| x.1.iter().map(|y| y.len()).min().unwrap())
        .min()
        .unwrap();
    let max_len = test_messages.iter().map(|x| x.len()).max().unwrap();
    let max_amount = max_len / min_sub_result_len;
    // TODO
    for eights in 1..=max_amount {
        let mut message = String::new();
        for elevens in 1..=max_amount {
            if test_messages.contains(&message.as_str()) {
                println!("{:?}", message);
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}

fn get_messages(start: usize, rules: &HashMap<usize, Vec<Vec<Entry>>>) -> Vec<String> {
    let mut results = Vec::new();
    let mut messages = vec![vec![Entry::Rule(start)]];
    while let Some(message) = messages.pop() {
        /* println!(
            "{:?}",
            message
                .iter()
                .map(|x| match x {
                    Entry::Literal(c) => c.to_string(),
                    Entry::Rule(r) => r.to_string(),
                })
                .collect::<String>()
        ); */
        let mut found_rule = false;
        for i in 0..message.len() {
            match message[i] {
                Entry::Literal(_) => (),
                Entry::Rule(rule) => {
                    found_rule = true;

                    for rule_part in rules[&rule].iter() {
                        let mut new_message = Vec::new();
                        new_message.extend_from_slice(&message[..i]);
                        for entry in rule_part.iter() {
                            new_message.push(entry.clone());
                        }
                        new_message.extend_from_slice(&message[i + 1..]);
                        messages.push(new_message);
                    }
                    break;
                }
            }
        }
        if !found_rule {
            let new_result = message
                .iter()
                .map(|x| match x {
                    Entry::Literal(c) => c,
                    _ => panic!(),
                })
                .collect::<String>();
            if !results.contains(&new_result) {
                results.push(new_result);
                /* if results.len() % 1000 == 0 {
                    println!("{}", results.len());
                } */
            }
        }
    }
    results
}
