pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut first = 0;
    let mut second = numbers.len() - 1;

    loop {
        let sum = numbers[first] + numbers[second];
        if sum > target {
            second -= 1;
        } else if sum < target {
            first += 1;
        } else {
            break;
        }
    }

    vec![first as i32 + 1, second as i32 + 1]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let expected_result = vec![1, 2];

    let result = two_sum(nums, 9);

    dbg!(&result);
    assert_eq!(expected_result, result);
}

// [3, 24, 50, 79, 88, 150, 345]
// 200