/*
Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:

Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
Return k.
*/

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut last = nums.len() - 1;
        let mut current: usize = 0;

        while current < last {
            if nums[current] == val {
                while last > 0 && nums[last] == val {
                    last -= 1;
                }

                if last <= current {
                    break;
                }

                nums[current] = nums[last];
                last -= 1;
            }
            current += 1;
        }

        if current == last && nums[current] != val {
            current += 1;
        }

        current as i32
    }
}
