pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut zeros_memory: Option<(usize, usize)> = None;

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 0 {
                if let Some((zeros_row, zeros_col)) = zeros_memory {
                    matrix[row][zeros_col] = 0;
                    matrix[zeros_row][col] = 0;
                } else {
                    zeros_memory = Some((row, col));
                }
            }
        }
    }

    if zeros_memory.is_none() {
        return;
    }

    let (zeros_row, zeros_col) = zeros_memory.unwrap();

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if row != zeros_row
                && col != zeros_col
                && (matrix[row][zeros_col] == 0 || matrix[zeros_row][col] == 0)
            {
                matrix[row][col] = 0;
            }
        }
    }

    for row in 0..matrix.len() {
        matrix[row][zeros_col] = 0;
    }
    for col in 0..matrix[0].len() {
        matrix[zeros_row][col] = 0;
    }
}

fn main() {
    let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    let expected_result = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];

    set_zeroes(&mut matrix);

    dbg!(&matrix);
    assert_eq!(expected_result, matrix);
}
