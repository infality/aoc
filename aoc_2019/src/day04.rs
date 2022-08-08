pub fn task1(input: &str) {
    let mut result = 0usize;

    let min: usize = input.split_once('-').unwrap().0.parse().unwrap();
    let max: usize = input.split_once('-').unwrap().1.parse().unwrap();

    for num in min..=max {
        let mut valid = false;
        for pos in 0..5 {
            if digit_at_pos(num, pos) < digit_at_pos(num, pos + 1) {
                valid = false;
                break;
            }
            if digit_at_pos(num, pos) == digit_at_pos(num, pos + 1) {
                valid = true;
            }
        }

        if valid {
            dbg!(num);
            result += 1;
        }
    }

    println!("Result: {}", result);
}

fn digit_at_pos(num: usize, pos: usize) -> usize {
    (num / 10_usize.pow(pos as u32)) % 10
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let min: usize = input.split_once('-').unwrap().0.parse().unwrap();
    let max: usize = input.split_once('-').unwrap().1.parse().unwrap();

    for num in min..=max {
        let mut valid = true;
        let mut nums = [0; 10];
        for pos in 0..5 {
            if digit_at_pos(num, pos) < digit_at_pos(num, pos + 1) {
                valid = false;
                break;
            }
            nums[digit_at_pos(num, pos)] += 1;
        }
        nums[digit_at_pos(num, 5)] += 1;

        if valid && nums.iter().any(|x| *x == 2) {
            dbg!(num);
            result += 1;
        }
    }

    println!("Result: {}", result);
}
