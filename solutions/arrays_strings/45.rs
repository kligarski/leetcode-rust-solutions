use std::cmp::min;

pub fn jump(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }

    let mut dp = Vec::with_capacity(nums.len());
    dp.push(0);
    for _ in 1..nums.len() {
        dp.push(i32::MAX);
    }

    for i in 0..nums.len() {
        let max_jump = nums[i];
        for j in (i + 1)..min(i + max_jump as usize + 1, nums.len()) {
            dp[j] = min(dp[j], dp[i] + 1);
        }
    }

    dp[nums.len() - 1]
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    let expected_result = 2;

    let result = jump(nums);
    assert_eq!(result, expected_result);
    dbg!(result);

    let nums = vec![2, 3, 0, 1, 4];
    let expected_result = 2;

    let result = jump(nums);
    assert_eq!(result, expected_result);
    dbg!(result);
}
