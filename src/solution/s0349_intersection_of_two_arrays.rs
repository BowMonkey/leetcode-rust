/**
 * [349] Intersection of Two Arrays
 *
 * Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must be unique and you may return the result in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums1 = [1,2,2,1], nums2 = [2,2]
 * Output: [2]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
 * Output: [9,4]
 * Explanation: [4,9] is also accepted.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 1000
 * 	0 <= nums1[i], nums2[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/intersection-of-two-arrays/
// discuss: https://leetcode.com/problems/intersection-of-two-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut mp: HashMap<i32, i32> = HashMap::new();
        //1. collect nums in nums1, set count to 1
        for i in 0..nums1.len() {
            mp.insert(nums1[i], 1);
        }
        //2. go through nums2
        for i in 0..nums2.len() {
            if let Some(&tmp) = mp.get(&nums2[i]) {
                if tmp == 1 {
                    res.push(nums2[i]);
                }
                mp.insert(nums2[i], tmp + 1);
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
    fn test_349() {}
}
