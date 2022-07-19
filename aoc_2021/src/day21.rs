pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut positions = input
        .lines()
        .map(|x| x.split_once(": ").unwrap().1.parse::<usize>().unwrap() - 1)
        .collect::<Vec<usize>>();

    let mut scores = vec![0, 0];

    let mut roll = 0;
    let mut dice_num = 1;
    let mut player = 0;
    while scores[0] < 1000 && scores[1] < 1000 {
        let mut rolled = dice_num;
        dice_num = (dice_num % 100) + 1;
        rolled += dice_num;
        dice_num = (dice_num % 100) + 1;
        rolled += dice_num;
        dice_num = (dice_num % 100) + 1;

        positions[player] = (positions[player] + rolled) % 10;
        scores[player] += positions[player] + 1;

        /* println!(
            "P{} rolled {} position {} score {}",
            player + 1,
            &rolled,
            positions[player] + 1,
            scores[player]
        ); */
        player = (player + 1) % 2;
        roll += 3;
    }
    println!("{:?}", scores);
    println!("{:?}", roll);

    result = scores[player] * roll;

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let input_pos = input
        .lines()
        .map(|x| x.split_once(": ").unwrap().1.parse::<usize>().unwrap() - 1)
        .collect::<Vec<usize>>();

    let mut wins = [0, 0];
    let mut positions = vec![[input_pos[0], input_pos[1]]];
    let mut scores = vec![[0, 0]];

    let mut rolls = vec![3];
    let mut player = 0;
    loop {
        let player_pos = (positions.last().unwrap()[player] + rolls.last().unwrap()) % 10;
        let player_score = scores.last().unwrap()[player] + player_pos + 1;

        if player_score >= 21 {
            let mut win_amount = 1;
            for roll in rolls.iter() {
                win_amount *= match roll {
                    3 => 1,
                    4 => 3,
                    5 => 6,
                    6 => 7,
                    7 => 6,
                    8 => 3,
                    9 => 1,
                    _ => panic!(),
                }
            }
            wins[player] += win_amount;
        } else {
            // Go deeper
            let mut new_pos = positions.last().unwrap().clone();
            new_pos[player] = player_pos;
            positions.push(new_pos);
            let mut new_score = scores.last().unwrap().clone();
            new_score[player] = player_score;
            scores.push(new_score);
            rolls.push(3);
            player = (player + 1) % 2;
            continue;
        }

        // Advance to next roll
        while !rolls.is_empty() && *rolls.last().unwrap() == 9 {
            rolls.pop();
            positions.pop();
            scores.pop();
            player = (player + 1) % 2;
        }
        if rolls.is_empty() {
            break;
        }
        *rolls.last_mut().unwrap() += 1;
    }

    println!("{:?}", wins);
    result = *wins.iter().max().unwrap();

    println!("Result: {}", result);
}
