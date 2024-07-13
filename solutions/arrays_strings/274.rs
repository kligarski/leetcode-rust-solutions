pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut min = 0;
    let mut max = citations.iter().max().copied().unwrap_or(0);

    let mut h = 0;
    while min <= max {
        let mid = (min + max) / 2;

        let count = citations.iter().filter(|&&val| val >= mid).count();

        if count >= mid as usize {
            min = mid + 1;
            h = mid;
        } else {
            max = mid - 1;
        }
    }

    h
}

fn main() {
    let nums = vec![3, 0, 6, 1, 5];
    let expected_result = 3;

    let result = h_index(nums);
    assert_eq!(result, expected_result);
    dbg!(result);

    let nums = vec![100];
    let expected_result = 1;

    let result = h_index(nums);
    assert_eq!(result, expected_result);
    dbg!(result);
}
