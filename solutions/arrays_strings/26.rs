/*
Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
Return k.
*/

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        } else if nums.len() == 1 {
            return 1;
        }

        let mut no_unique: usize = 1;
        let mut current: usize = 1;

        'outer: while current < nums.len() {
            while nums[current] == nums[current - 1] {
                current += 1;
                if current >= nums.len() {
                    break 'outer;
                }
            }

            nums[no_unique] = nums[current];

            no_unique += 1;
            current += 1;
        }

        no_unique as i32
    }
}
