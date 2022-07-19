pub fn task1(input: &str) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let chosen_numbers: Vec<&str> = parts[0].split(",").collect();
    let chosen_numbers: Vec<usize> = chosen_numbers.iter().map(|x| x.parse().unwrap()).collect();

    let mut boards = Vec::new();
    let mut markers = Vec::new();

    for part in parts.iter().skip(1) {
        let mut board = [[0usize; 5]; 5];
        for (row, line) in part.split("\n").enumerate() {
            for (col, field) in line.split(" ").filter(|x| *x != "").enumerate() {
                board[row][col] = field.parse().unwrap();
            }
        }
        boards.push(board);
        markers.push([[false; 5]; 5]);
    }

    let mut winning_board = 0;
    let mut winning_number = 0;

    'outer: for chosen_number in chosen_numbers.iter() {
        for (i, board) in boards.iter().enumerate() {
            for row in 0..5 {
                for col in 0..5 {
                    if &board[row][col] == chosen_number {
                        markers[i][row][col] = true;
                        let m = markers[i];

                        if m[row][0] && m[row][1] && m[row][2] && m[row][3] && m[row][4] {
                            winning_board = i;
                            winning_number = *chosen_number;
                            break 'outer;
                        }
                        if m[0][col] && m[1][col] && m[2][col] && m[3][col] && m[4][col] {
                            winning_board = i;
                            winning_number = *chosen_number;
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;
    for row in 0..5 {
        for col in 0..5 {
            if !markers[winning_board][row][col] {
                result += boards[winning_board][row][col];
            }
        }
    }

    println!("Result: {}", result * winning_number);
}

pub fn task2(input: &str) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let chosen_numbers: Vec<&str> = parts[0].split(",").collect();
    let chosen_numbers: Vec<usize> = chosen_numbers.iter().map(|x| x.parse().unwrap()).collect();

    let mut boards = Vec::new();
    let mut markers = Vec::new();

    for part in parts.iter().skip(1) {
        let mut board = [[0usize; 5]; 5];
        for (row, line) in part.split("\n").enumerate() {
            for (col, field) in line.split(" ").filter(|x| *x != "").enumerate() {
                board[row][col] = field.parse().unwrap();
            }
        }
        boards.push(board);
        markers.push([[false; 5]; 5]);
    }

    let mut loosing_board = 0;
    let mut loosing_number = 0;
    let mut boards_finished = Vec::new();

    'outer: for chosen_number in chosen_numbers.iter() {
        for (i, board) in boards.iter().enumerate() {
            for row in 0..5 {
                for col in 0..5 {
                    if !boards_finished.contains(&i) && &board[row][col] == chosen_number {
                        markers[i][row][col] = true;
                        let m = markers[i];

                        if (m[row][0] && m[row][1] && m[row][2] && m[row][3] && m[row][4])
                            || (m[0][col] && m[1][col] && m[2][col] && m[3][col] && m[4][col])
                        {
                            if boards_finished.len() < boards.len() - 1 {
                                boards_finished.push(i);
                                continue;
                            }
                            loosing_board = i;
                            loosing_number = *chosen_number;
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;
    for row in 0..5 {
        for col in 0..5 {
            if !markers[loosing_board][row][col] {
                result += boards[loosing_board][row][col];
            }
        }
    }

    println!("Result: {}", result * loosing_number);
}
