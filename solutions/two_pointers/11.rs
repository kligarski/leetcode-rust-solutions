use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = height.len() - 1;

    let mut current_max = 0;
    while i < j {
        current_max = max(current_max, (j - i) as i32 * min(height[i], height[j]));
        if height[i] <= height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }

    current_max
}

fn main() {
    let nums = vec![1,8,6,2,5,4,8,3,7];
    let expected_result = 49;

    let result = max_area(nums);

    dbg!(&result);
    assert_eq!(expected_result, result);
}