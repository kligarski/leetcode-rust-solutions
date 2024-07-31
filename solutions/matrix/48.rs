fn two_dim_swap(matrix: &mut Vec<Vec<i32>>, x1: usize, y1: usize, x2: usize, y2: usize) {
    let tmp = matrix[x2][y2];
    matrix[x2][y2] = matrix[x1][y1];
    matrix[x1][y1] = tmp;
}

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for level in 0..(n / 2) {
        for pos in level..(n - level - 1) {
            two_dim_swap(matrix, level, pos, pos, n - level - 1);
            two_dim_swap(matrix, level, pos, n - level - 1, n - pos - 1);
            two_dim_swap(matrix, level, pos, n - pos - 1, level);
        }
    }
}

fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected_result = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

    rotate(&mut matrix);

    dbg!(&matrix);
    assert_eq!(expected_result, matrix);
}
