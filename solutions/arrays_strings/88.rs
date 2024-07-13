/*
You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

Merge nums1 and nums2 into a single array sorted in non-decreasing order.

The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
*/

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut result: Vec<i32> = Vec::with_capacity((m + n) as usize);

        let m: usize = m as usize;
        let n: usize = n as usize;

        let mut i: usize = 0;
        let mut j: usize = 0;

        while i < m && j < n {
            if nums1[i] <= nums2[j] {
                result.push(nums1[i]);
                i += 1;
            } else {
                result.push(nums2[j]);
                j += 1;
            }
        }

        while i < m {
            result.push(nums1[i]);
            i += 1;
        }

        while j < n {
            result.push(nums2[j]);
            j += 1;
        }

        for i in 0..(m + n) {
            nums1[i] = result[i];
        }
    }
}
