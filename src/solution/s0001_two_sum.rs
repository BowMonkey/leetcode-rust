/**
 * [1] Two Sum
 *
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * You can return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [3,2,4], target = 6
 * Output: [1,2]
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = [3,3], target = 6
 * Output: [0,1]
 * 
 *  
 * Constraints:
 * 
 * 	2 <= nums.length <= 10^4
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 * 	Only one valid answer exists.
 * 
 *  
 * Follow-up: Can you come up with an algorithm that is less than O(n^2) time complexity?
 */

/*
思路
1. 建立哈希表存储（数字，索引），同时遍历数组
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum/
// discuss: https://leetcode.com/problems/two-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::{collections::HashMap, hash::Hash};
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp:HashMap<i32,i32>=HashMap::new();
        for (idx, &n) in nums.iter().enumerate(){
            let other=target-n;
            if mp.contains_key(&other){
                return vec![ *(mp.get(&other).unwrap()), idx as i32];
            }
            mp.insert(n, idx as i32);
            
        }
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0,1]);
    }

    #[test]
    fn test_1_2() {
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1,2]);
    }

    #[test]
    fn test_1_3() {
        assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0,1]);
    }
}
