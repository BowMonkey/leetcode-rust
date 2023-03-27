/**
 * [1005] Maximize Sum Of Array After K Negations
 *
 * Given an integer array nums and an integer k, modify the array in the following way:
 *
 * 	choose an index i and replace nums[i] with -nums[i].
 *
 * You should apply this process exactly k times. You may choose the same index i multiple times.
 * Return the largest possible sum of the array after modifying it in this way.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [4,2,3], k = 1
 * Output: 5
 * Explanation: Choose index 1 and nums becomes [4,-2,3].
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,-1,0,2], k = 3
 * Output: 6
 * Explanation: Choose indices (1, 2, 2) and nums becomes [3,1,0,2].
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [2,-3,-1,5,-4], k = 2
 * Output: 13
 * Explanation: Choose indices (1, 4) and nums becomes [2,3,-1,5,4].
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-100 <= nums[i] <= 100
 * 	1 <= k <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/
// discuss: https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn largest_sum_after_k_negations_magic(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = nums.into_iter().map(Reverse).collect::<BinaryHeap<_>>();
        for _ in 0..k {
            let top = Reverse(-heap.pop().unwrap().0);
            heap.push(top);
        }
        heap.iter().map(|i| i.0).sum()
    }
}
impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort();
        for idx in 0..nums.len(){
            if k>0 && nums[idx]<0{
                nums[idx] *= -1;
                k-=1;
            }
        }
        nums.sort();
        if k>0 && k%2==1 && nums[0]!=0{
            nums[0] *=-1;
        }
        nums.iter().sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1005_1() {
        assert_eq!(Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
    }
    #[test]
    fn test_1005_2() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3),
            6
        );
    }
    #[test]
    fn test_1005_3() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2),
            13
        );
    }
}
