use std::cmp::max;

pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }

    let mut i = 0;
    let mut available_jumps = 0;

    while i < nums.len() - 1 {
        available_jumps = max(nums[i], available_jumps);
        if available_jumps == 0 {
            break;
        }
        i += 1;
        available_jumps -= 1;
    }

    i == nums.len() - 1
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    let expected_result = true;

    let result = can_jump(nums);
    assert_eq!(result, expected_result);
    dbg!(result);

    let nums = vec![3, 2, 1, 0, 4];
    let expected_result = false;

    let result = can_jump(nums);
    assert_eq!(result, expected_result);
    dbg!(result);
}
