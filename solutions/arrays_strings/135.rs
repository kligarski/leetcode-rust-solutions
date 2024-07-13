use std::cmp::max;

pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut candies = vec![1; n];

    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }

    for j in (0..n - 1).rev() {
        if ratings[j] > ratings[j + 1] {
            candies[j] = max(candies[j], candies[j + 1] + 1);
        }
    }

    candies.iter().sum()
}

fn main() {
    let nums = vec![1, 0, 2];
    let expected_result = 5;

    let result = candy(nums);

    dbg!(&result);
    assert_eq!(expected_result, result);
}

// 1 0 2
// 1 1 2
// 2 1 1
// 2 1 2

// 0 5 1 2 3 4 4 4 2 0
// 1 2 1 2 3 4 1 1 1 1 left
// 1 2 1 1 1 1 1 3 2 1 right
// 1 2 1 2 3 4 1 3 2 1 max(left, right)

// 1 2 1 2 3 4 1 3 2 1
