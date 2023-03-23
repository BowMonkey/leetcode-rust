/**
 * [35] Search Insert Position
 *
 * Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
 * You must write an algorithm with O(log n) runtime complexity.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [1,3,5,6], target = 5
 * Output: 2
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,3,5,6], target = 2
 * Output: 1
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = [1,3,5,6], target = 7
 * Output: 4
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 	nums contains distinct values sorted in ascending order.
 * 	-10^4 <= target <= 10^4
 * 
 */

/*
思路
1. 遍历数组即可

 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-insert-position/
// discuss: https://leetcode.com/problems/search-insert-position/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for (idx,val) in nums.iter().enumerate() {
            if *val >= target {
                return idx as i32;
            }
        }
        nums.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_35_1() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6],5), 2);
    }
    #[test]
    fn test_35_2() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6],2), 1);
    }
    #[test]
    fn test_35_3() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6],7), 4);
    }
    #[test]
    fn test_35_4() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6],1), 0);
    }
    #[test]
    fn test_35_5() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6],0), 0);
    }
}
