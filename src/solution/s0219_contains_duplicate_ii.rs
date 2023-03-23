/**
 * [219] Contains Duplicate II
 *
 * Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,3,1], k = 3
 * Output: true
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,0,1,1], k = 1
 * Output: true
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,2,3,1,2,3], k = 2
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	0 <= k <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/contains-duplicate-ii/
// discuss: https://leetcode.com/problems/contains-duplicate-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut lookup: HashMap<i32, usize> = HashMap::new();
        for (idx, n) in nums.iter().enumerate() {
            if let Some(tmp_idx) = lookup.get(n) {
                if idx - tmp_idx <= k as usize {
                    return true;
                }
            }
            lookup.insert(*n, idx);
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_219() {}
}
