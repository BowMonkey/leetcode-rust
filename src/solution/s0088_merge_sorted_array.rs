/**
 * [88] Merge Sorted Array
 *
 * You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
 * Merge nums1 and nums2 into a single array sorted in non-decreasing order.
 * The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
 * Output: [1,2,2,3,5,6]
 * Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
 * The result of the merge is [<u>1</u>,<u>2</u>,2,<u>3</u>,5,6] with the underlined elements coming from nums1.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums1 = [1], m = 1, nums2 = [], n = 0
 * Output: [1]
 * Explanation: The arrays we are merging are [1] and [].
 * The result of the merge is [1].
 *
 * <strong class="example">Example 3:
 *
 * Input: nums1 = [0], m = 0, nums2 = [1], n = 1
 * Output: [1]
 * Explanation: The arrays we are merging are [] and [1].
 * The result of the merge is [1].
 * Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.
 *
 *  
 * Constraints:
 *
 * 	nums1.length == m + n
 * 	nums2.length == n
 * 	0 <= m, n <= 200
 * 	1 <= m + n <= 200
 * 	-10^9 <= nums1[i], nums2[j] <= 10^9
 *
 *  
 * Follow up: Can you come up with an algorithm that runs in O(m + n) time?
 *
 */

/*
思路
1. m+n是第一个数组的长度，双指针，从后往前遍历，第三个指针指示最大元素数组
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-sorted-array/
// discuss: https://leetcode.com/problems/merge-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i, mut j) = (m - 1, n - 1);
        let mut idx = m + n - 1;
        let mut bigger = 0;
        while (i >= 0 && j >= 0) {
            if nums1[i as usize] >= nums2[j as usize] {
                bigger = nums1[i as usize];
                i -= 1;
            } else {
                bigger = nums2[j as usize];
                j -= 1;
            }
            nums1[idx as usize] = bigger;
            idx -= 1;
        }
        while (j >= 0) {
            nums1[idx as usize] = nums2[j as usize];
            j -= 1;
            idx -= 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88_1() {
        let mut v1 = vec![1, 2, 3, 0, 0, 0];
        let mut v2 = vec![2, 5, 6];
        Solution::merge(&mut v1, 3, &mut v2, 3);
        assert_eq!(v1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_88_2() {
        let mut v1 = vec![5, 6, 0, 0, 0];
        let mut v2 = vec![-1, 2, 3];
        Solution::merge(&mut v1, 2, &mut v2, 3);
        assert_eq!(v1, vec![-1, 2, 3, 5, 6]);
    }

    #[test]
    fn test_88_3() {
        let mut v1 = vec![0];
        let mut v2 = vec![-1];
        Solution::merge(&mut v1, 0, &mut v2, 1);
        assert_eq!(v1, vec![-1]);
    }
}
