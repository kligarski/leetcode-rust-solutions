/*
Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice. The relative order of the elements should be kept the same.

Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.

Return k after placing the final result in the first k slots of nums.

Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
*/

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        } else if nums.len() == 1 {
            return 1;
        }

        let mut current: usize = 1;

        while current < nums.len() && nums[current] == nums[current - 1] && current < 2 {
            current += 1;
        }

        let mut next_start = current;
        while next_start < nums.len() && nums[next_start] == nums[current - 1] {
            next_start += 1;
        }

        while next_start < nums.len() {
            let mut sequence_len: usize = 1;
            while next_start + sequence_len < nums.len()
                && nums[next_start + sequence_len] == nums[next_start]
            {
                sequence_len += 1;
            }

            if sequence_len == 1 {
                nums[current] = nums[next_start];
                current += 1;
                next_start += 1;
            } else if sequence_len > 1 {
                nums[current] = nums[next_start];
                nums[current + 1] = nums[next_start];
                current += 2;
                next_start += sequence_len;
            }
        }

        current as i32
    }
}
