pub fn task1(input: &str) {
    let mut result = 0isize;

    let histories: Vec<Vec<isize>> = input
        .lines()
        .map(|x| x.split_whitespace().map(|y| y.parse().unwrap()).collect())
        .collect();

    for history in histories {
        let mut seqs = Vec::new();
        seqs.push(history);
        while seqs.last().unwrap().iter().any(|y| *y != 0) {
            let mut seq = Vec::new();
            for window in seqs.last().unwrap().windows(2) {
                seq.push(window[1] - window[0]);
            }
            seqs.push(seq);
        }

        seqs.last_mut().unwrap().push(0);
        for i in (0..seqs.len() - 1).rev() {
            let j = seqs[i].len() - 1;
            let value = seqs[i][j] + seqs[i + 1][j];
            seqs[i].push(value);
        }
        result += *seqs.first().unwrap().last().unwrap();
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0isize;

    let histories: Vec<Vec<isize>> = input
        .lines()
        .map(|x| x.split_whitespace().map(|y| y.parse().unwrap()).collect())
        .collect();

    for history in histories {
        let mut seqs = Vec::new();
        seqs.push(history);
        while seqs.last().unwrap().iter().any(|y| *y != 0) {
            let mut seq = Vec::new();
            for window in seqs.last().unwrap().windows(2) {
                seq.push(window[1] - window[0]);
            }
            seqs.push(seq);
        }

        seqs.last_mut().unwrap().push(0);
        for i in (0..seqs.len() - 1).rev() {
            let value = seqs[i][0] - seqs[i + 1].last().unwrap();
            seqs[i].push(value);
        }
        result += *seqs.first().unwrap().last().unwrap();
    }

    println!("Result: {}", result);
}
