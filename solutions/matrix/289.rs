const DX: [i32; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
const DY: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

// -2: 1->0
// -1: 0->0
//  0: 0
//  1: 1
//  2: 1->1
//  3: 0->1

fn get_neighbour_at(board: &Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
    if x < 0 || y < 0 || x >= board.len() as i32 || y >= board[0].len() as i32 {
        return 0;
    }
    match board[x as usize][y as usize] {
        -2 | 1 | 2 => 1,
        _ => 0,
    }
}

fn no_neighbours(board: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut answer: i32 = 0;
    for move_index in 0..DX.len() {
        answer += get_neighbour_at(board, x as i32 + DX[move_index], y as i32 + DY[move_index]);
    }

    answer
}

pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let neighbours = no_neighbours(board, i, j);
            board[i][j] = match (board[i][j], neighbours) {
                (1, n) if n < 2 => -2,
                (1, n) if n < 4 => 2,
                (1, _) => -2,
                (_, 3) => 3,
                (_, _) => 0,
            }
        }
    }

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            board[i][j] = match board[i][j] {
                -2 | -1 | 0 => 0,
                _ => 1,
            }
        }
    }
}

fn main() {
    let mut matrix = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    let expected_result = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];

    game_of_life(&mut matrix);

    dbg!(&matrix);
    assert_eq!(expected_result, matrix);
}
