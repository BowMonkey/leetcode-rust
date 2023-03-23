/**
 * [169] Majority Element
 *
 * Given an array nums of size n, return the majority element.
 * The majority element is the element that appears more than &lfloor;n / 2&rfloor; times. You may assume that the majority element always exists in the array.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [3,2,3]
 * Output: 3
 * <strong class="example">Example 2:
 * Input: nums = [2,2,1,1,1,2,2]
 * Output: 2
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 5 * 10^4
 * 	-10^9 <= nums[i] <= 10^9
 *
 *  
 * Follow-up: Could you solve the problem in linear time and in O(1) space?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/majority-element/
// discuss: https://leetcode.com/problems/majority-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut num = 0;
        for i in 0..nums.len() {
            if cnt == 0 {
                num = nums[i];
                cnt += 1;
            } else {
                if nums[i] == num {
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
            }
        }
        num
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_169() {}
}
