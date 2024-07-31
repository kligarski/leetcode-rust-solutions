fn check_if_box_valid(board: &Vec<Vec<char>>, digits: &mut [bool; 10], x: usize, y: usize) -> bool {
    if let Some(digit) = board[x][y].to_digit(10) {
        let digit = digit as usize;
        if digits[digit] {
            return false;
        } else {
            digits[digit] = true;
        }
    }
    true
}

fn check_subbox(board: &Vec<Vec<char>>, subbox_x: usize, subbox_y: usize) -> bool {
    assert!(subbox_x < 3);
    assert!(subbox_y < 3);

    let mut digits: [bool; 10] = [false; 10];
    for row in 0..3 {
        for col in 0..3 {
            let x = subbox_x * 3 + row;
            let y = subbox_y * 3 + col;

            if !check_if_box_valid(board, &mut digits, x, y) {
                return false;
            }
        }
    }
    true
}

fn check_line(board: &Vec<Vec<char>>, checked_index: usize, horizontal: bool) -> bool {
    assert!(checked_index < 9);

    let mut digits: [bool; 10] = [false; 10];
    for pos in 0..9 {
        let x = if horizontal { checked_index } else { pos };
        let y = if horizontal { pos } else { checked_index };

        if !check_if_box_valid(board, &mut digits, x, y) {
            return false;
        }
    }
    true
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for subbox_x in 0..3 {
        for subbox_y in 0..3 {
            if !check_subbox(&board, subbox_x, subbox_y) {
                return false;
            }
        }
    }

    for row in 0..9 {
        if !check_line(&board, row, true) {
            return false;
        }
    }

    for col in 0..9 {
        if !check_line(&board, col, false) {
            return false;
        }
    }
    true
}

fn main() {
    let board = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let expected_result = false;

    let result = is_valid_sudoku(board);

    dbg!(&result);
    assert_eq!(expected_result, result);
}
