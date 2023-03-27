/**
 * [1827] Minimum Operations to Make the Array Increasing
 *
 * You are given an integer array nums (0-indexed). In one operation, you can choose an element of the array and increment it by 1.
 *
 *
 * 	For example, if nums = [1,2,3], you can choose to increment nums[1] to make nums = [1,<u>3</u>,3].
 *
 *
 * Return the minimum number of operations needed to make nums strictly increasing.
 *
 * An array nums is strictly increasing if nums[i] < nums[i+1] for all 0 <= i < nums.length - 1. An array of length 1 is trivially strictly increasing.
 *
 *  
 * <strong class="example">Example 1:
 *
 *
 * Input: nums = [1,1,1]
 * Output: 3
 * Explanation: You can do the following operations:
 * 1) Increment nums[2], so nums becomes [1,1,<u>2</u>].
 * 2) Increment nums[1], so nums becomes [1,<u>2</u>,2].
 * 3) Increment nums[2], so nums becomes [1,2,<u>3</u>].
 *
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: nums = [1,5,2,4,1]
 * Output: 14
 *
 *
 * <strong class="example">Example 3:
 *
 *
 * Input: nums = [8]
 * Output: 0
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= nums.length <= 5000
 * 	1 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-operations-to-make-the-array-increasing/
// discuss: https://leetcode.com/problems/minimum-operations-to-make-the-array-increasing/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for idx in 1..nums.len() {
            if nums[idx] - nums[idx - 1] <= 0 {
                let mut tmp = nums[idx - 1] - nums[idx] + 1;
                nums[idx] += tmp;
                res += tmp;
            }
        }
        res
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1827() {}
}
