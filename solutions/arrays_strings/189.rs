fn mirror(nums: &mut Vec<i32>, left: usize, right: usize) {
    let length = right - left + 1;

    for delta in 0..(length / 2) {
        nums.swap(left + delta, right - delta);
    }
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    let k: usize = k as usize % n;

    mirror(nums, 0, n - k - 1);
    mirror(nums, n - k, n - 1);

    mirror(nums, 0, n - 1);
}


fn main() {
    let mut nums = vec![1,2,3,4,5,6,7];
    let k = 3;
    let expected_nums = vec![5,6,7,1,2,3,4];

    rotate(&mut nums, k);
    assert_eq!(nums, expected_nums);
    dbg!(nums);
}
