/**
 * [283] Move Zeroes
 *
 * Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
 * Note that you must do this in-place without making a copy of the array.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [0,1,0,3,12]
 * Output: [1,3,12,0,0]
 * <strong class="example">Example 2:
 * Input: nums = [0]
 * Output: [0]
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 *
 *  
 * Follow up: Could you minimize the total number of operations done?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/move-zeroes/
// discuss: https://leetcode.com/problems/move-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut end = 0;
        for it in 0..nums.len() {
            if nums[it] != 0 {
                nums[end] = nums[it];
                if it != end {
                    nums[it] = 0;
                }
                end += 1;
            }
        }
    }
    pub fn move_zeroes_1(nums: &mut Vec<i32>) {
        let mut start = 0;
        let mut end = 0;
        while end < nums.len() {
            while start < nums.len() && nums[start] != 0 {
                start += 1;
            }
            if start >= nums.len() {
                return;
            }
            end = start;
            while end < nums.len() && nums[end] == 0 {
                end += 1;
            }
            if end >= nums.len() {
                return;
            }
            nums.swap(start, end);
            start += 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_283() {
        let mut v = vec![0, 1, 0, 3, 12];
        let res = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, res);
    }
}
