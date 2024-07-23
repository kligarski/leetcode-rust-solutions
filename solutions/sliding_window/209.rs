pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = 0;

    let mut answer = nums.len() + 1;
    let mut current_sum = nums[0];

    while left < nums.len() && right < nums.len() {
        if current_sum >= target {
            answer = std::cmp::min(answer, right - left + 1);
        }
        
        if current_sum < target {
            right += 1;

            if right >= nums.len() {
                break;
            }

            current_sum += nums[right];
        } else if left == right {
            left += 1;
            right += 1;

            if right >= nums.len() {
                break;
            }

            current_sum = nums[right];
        } else {
            current_sum -= nums[left];
            left += 1;
        }
    }

    if answer > nums.len() {
        0
    } else {
        answer as i32
    }
}

fn main() {
    let nums = vec![2, 3, 1, 2, 4, 3];
    let expected_result = 2;

    let result = min_sub_array_len(7, nums);

    dbg!(&result);
    assert_eq!(expected_result, result);
}
