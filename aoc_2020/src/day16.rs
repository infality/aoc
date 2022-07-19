use std::collections::HashSet;

pub fn task1(input: &str) {
    let mut result = 0usize;
    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 1usize;

    let input_parts = input.splitn(3, "\n\n").collect::<Vec<&str>>();
    let mut rules: Vec<[(usize, usize); 2]> = Vec::new();
    for line in input_parts[0].lines() {
        let rule = line.split_once(": ").unwrap().1.split_once(" or ").unwrap();
        let left = rule.0.split_once('-').unwrap();
        let right = rule.1.split_once('-').unwrap();
        rules.push([
            (left.0.parse().unwrap(), left.1.parse().unwrap()),
            (right.0.parse().unwrap(), right.1.parse().unwrap()),
        ]);
    }

    let my_ticket = input_parts[1]
        .split_once('\n')
        .unwrap()
        .1
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut other_tickets = Vec::new();
    for line in input_parts[2].lines().skip(1) {
        other_tickets.push(
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    for i in (0..other_tickets.len()).rev() {
        if other_tickets[i]
            .iter()
            .any(|x| rules.iter().all(|y| !is_in_range(y, x)))
        {
            other_tickets.remove(i);
        }
    }

    let mut possible_mappings = Vec::new();
    for i in 0..rules.len() {
        possible_mappings.push(Vec::new());
        for j in 0..rules.len() {
            possible_mappings[i].push(j);
        }
    }

    for other_ticket in other_tickets.iter() {
        for (i, value) in other_ticket.iter().enumerate() {
            possible_mappings[i].retain(|x| is_in_range(&rules[*x], value));
        }
    }

    let mut fixed_rules: HashSet<usize> = HashSet::from_iter(0..rules.len());
    let mut changed = true;
    while changed {
        changed = false;
        let rule = possible_mappings
            .iter()
            .position(|x| x.len() == 1 && fixed_rules.contains(&x[0]));

        if let Some(rule) = rule {
            let value = possible_mappings[rule][0];
            for (j, mapping) in possible_mappings.iter_mut().enumerate() {
                if rule == j {
                    continue;
                }
                mapping.retain(|x| *x != value);
            }
            fixed_rules.remove(&value);
            changed = true;
        }
    }

    println!("{:?}", possible_mappings);
    for i in 0..possible_mappings.len() {
        if possible_mappings[i][0] < 6 {
            result *= my_ticket[i];
        }
    }

    println!("Result: {}", result);
}

fn is_in_range(rule: &[(usize, usize); 2], value: &usize) -> bool {
    (value >= &rule[0].0 && value <= &rule[0].1) || (value >= &rule[1].0 && value <= &rule[1].1)
}
