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

    // <42>+(<42><31>)+
    for test_message in test_messages.iter() {
        let mut messages = Vec::new();
        for sub_result in sub_results[&42].iter() {
            if test_message.starts_with(sub_result) {
                messages.push((sub_result.clone(), 0));
            }
        }

        let mut i = 0;
        while i < messages.len() {
            for sub_result in sub_results[&42].iter() {
                if test_message[messages[i].0.len()..].starts_with(sub_result) {
                    messages.push((format!("{}{}", messages[i].0, sub_result), 0));
                }
            }
            i += 1;
        }

        let mut i = 0;
        while i < messages.len() {
            for sub_result in sub_results[&42].iter() {
                if test_message[messages[i].0.len()..].starts_with(sub_result) {
                    messages.push((
                        format!("{}{}", messages[i].0, sub_result),
                        messages[i].1 + 1,
                    ));
                }
            }
            i += 1;
        }

        messages.retain(|x| x.1 != 0);

        let mut i = 0;
        while i < messages.len() {
            if messages[i].1 == 0 {
                i += 1;
                continue;
            }
            for sub_result in sub_results[&31].iter() {
                if test_message[messages[i].0.len()..].starts_with(sub_result) {
                    messages.push((
                        format!("{}{}", messages[i].0, sub_result),
                        messages[i].1 - 1,
                    ));
                }
            }
            i += 1;
        }

        for message in messages.iter() {
            if message.1 == 0 && test_message == &message.0 {
                result += 1;
                println!("{}", test_message);
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
