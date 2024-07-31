// pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
//     let mut left: i32 = 0;
//     let mut right: i32 = (matrix[0].len() - 1) as i32;
//     let mut top: i32 = 0;
//     let mut bottom: i32 = (matrix.len() - 1) as i32;

//     let mut x: i32 = 0;
//     let mut y: i32 = 0;

//     let mut answer = vec![matrix[0][0]];
//     let mut horizontal = false;
//     loop {
//         let prev_x = x;
//         let prev_y = y;

//         if horizontal {
//             break;
//         }
//         while y < right {
//             y += 1;
//             answer.push(matrix[x as usize][y as usize]);
//             horizontal = true;
//         }
//         top += 1;

//         if !horizontal && answer.len() != 1 {
//             break;
//         }
//         while x < bottom {
//             x += 1;
//             answer.push(matrix[x as usize][y as usize]);
//             horizontal = false;
//         }
//         right -= 1;

//         if horizontal {
//             break;
//         }
//         while y > left {
//             y -= 1;
//             answer.push(matrix[x as usize][y as usize]);
//             horizontal = true;
//         }
//         bottom -= 1;

//         if !horizontal {
//             break;
//         }
//         while x > top {
//             x -= 1;
//             answer.push(matrix[x as usize][y as usize]);
//             horizontal = false;
//         }
//         left += 1;

//         if prev_x == x && prev_y == y {
//             break;
//         }
//     }

//     answer
// }

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut left: i32 = 0;
    let mut right: i32 = (matrix[0].len() - 1) as i32;
    let mut top: i32 = 0;
    let mut bottom: i32 = (matrix.len() - 1) as i32;

    let mut answer = vec![];
    while left <= right && top <= bottom {
        for j in left..=right {
            answer.push(matrix[top as usize][j as usize]);
        }
        top += 1;

        for i in top..=bottom {
            answer.push(matrix[i as usize][right as usize])
        }
        right -= 1;

        if top <= bottom {
            for j in (left..=right).rev() {
                answer.push(matrix[bottom as usize][j as usize]);
            }
        }
        bottom -= 1;

        if left <= right {
            for i in (top..=bottom).rev() {
                answer.push(matrix[i as usize][left as usize]);
            }
        }
        left += 1;
    }

    answer
}

fn main() {
    let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    let expected_result = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];

    let result = spiral_order(matrix);

    dbg!(&result);
    assert_eq!(expected_result, result);

    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected_result = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];

    let result = spiral_order(matrix);

    dbg!(&result);
    assert_eq!(expected_result, result);

    let matrix = vec![vec![1]];
    let expected_result = vec![1];

    let result = spiral_order(matrix);

    dbg!(&result);
    assert_eq!(expected_result, result);

    let matrix = vec![vec![3, 2]];
    let expected_result = vec![3, 2];

    let result = spiral_order(matrix);

    dbg!(&result);
    assert_eq!(expected_result, result);

    let matrix = vec![vec![3], vec![2]];
    let expected_result = vec![3, 2];

    let result = spiral_order(matrix);

    dbg!(&result);
    assert_eq!(expected_result, result);
}
