use std::collections::HashMap;

fn get_type(hand: &str) -> usize {
    assert!(hand.len() == 5);
    let mut card_counts = HashMap::new();
    for card in hand.chars() {
        card_counts.entry(card).and_modify(|x| *x += 1).or_insert(1);
    }

    if card_counts.values().any(|x| *x == 5) {
        return 6;
    }

    if card_counts.values().any(|x| *x == 4) {
        return 5;
    }

    if card_counts.values().any(|x| *x == 3) && card_counts.values().any(|x| *x == 2) {
        return 4;
    }

    if card_counts.values().any(|x| *x == 3) {
        return 3;
    }

    if card_counts.values().filter(|x| **x == 2).count() == 2 {
        return 2;
    }

    if card_counts.values().any(|x| *x == 2) {
        return 1;
    }
    0
}

fn get_rank(card: char) -> usize {
    if card.is_digit(10) {
        return card.to_digit(10).unwrap() as usize;
    }
    match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut hands: Vec<(&str, usize)> = input
        .lines()
        .map(|x| x.trim().split_once(" ").unwrap())
        .map(|x| (x.0, x.1.parse().unwrap()))
        .collect();

    hands.sort_unstable_by(|a, b| {
        if get_type(a.0) != get_type(b.0) {
            return get_type(a.0).cmp(&get_type(b.0));
        }
        for (ac, bc) in a.0.chars().zip(b.0.chars()) {
            if ac == bc {
                continue;
            }
            return get_rank(ac).cmp(&get_rank(bc));
        }
        std::cmp::Ordering::Equal
    });

    for (i, hand) in hands.iter().enumerate() {
        result += (i + 1) * hand.1;
    }

    println!("Result: {}", result);
}

fn get_type2(hand: &str) -> usize {
    assert!(hand.len() == 5);
    let mut card_counts = HashMap::new();
    for card in hand.chars() {
        card_counts.entry(card).and_modify(|x| *x += 1).or_insert(1);
    }

    if card_counts.keys().any(|x| *x != 'J') {
        if let Some(j_count) = card_counts.remove(&'J') {
            *card_counts.iter_mut().max_by_key(|x| *x.1).unwrap().1 += j_count;
        }
    }

    if card_counts.values().any(|x| *x == 5) {
        return 6;
    }

    if card_counts.values().any(|x| *x == 4) {
        return 5;
    }

    if card_counts.values().any(|x| *x == 3) && card_counts.values().any(|x| *x == 2) {
        return 4;
    }

    if card_counts.values().any(|x| *x == 3) {
        return 3;
    }

    if card_counts.values().filter(|x| **x == 2).count() == 2 {
        return 2;
    }

    if card_counts.values().any(|x| *x == 2) {
        return 1;
    }
    0
}

fn get_rank2(card: char) -> usize {
    if card.is_digit(10) {
        return card.to_digit(10).unwrap() as usize;
    }
    match card {
        'T' => 10,
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut hands: Vec<(&str, usize)> = input
        .lines()
        .map(|x| x.trim().split_once(" ").unwrap())
        .map(|x| (x.0, x.1.parse().unwrap()))
        .collect();

    hands.sort_unstable_by(|a, b| {
        if get_type2(a.0) != get_type2(b.0) {
            return get_type2(a.0).cmp(&get_type2(b.0));
        }
        for (ac, bc) in a.0.chars().zip(b.0.chars()) {
            if ac == bc {
                continue;
            }
            return get_rank2(ac).cmp(&get_rank2(bc));
        }
        std::cmp::Ordering::Equal
    });

    for (i, hand) in hands.iter().enumerate() {
        result += (i + 1) * hand.1;
    }

    println!("Result: {}", result);
}
