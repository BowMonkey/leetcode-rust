/**
 * [350] Intersection of Two Arrays II
 *
 * Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums1 = [1,2,2,1], nums2 = [2,2]
 * Output: [2,2]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
 * Output: [4,9]
 * Explanation: [9,4] is also accepted.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums1.length, nums2.length <= 1000
 * 	0 <= nums1[i], nums2[i] <= 1000
 *
 *  
 * Follow up:
 *
 * 	What if the given array is already sorted? How would you optimize your algorithm?
 * 	What if nums1's size is small compared to nums2's size? Which algorithm is better?
 * 	What if elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/intersection-of-two-arrays-ii/
// discuss: https://leetcode.com/problems/intersection-of-two-arrays-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut mp: HashMap<i32, i32> = HashMap::new();
        //1. collect nums in nums1, record counts
        for i in 0..nums1.len() {
            match mp.get(&nums1[i]) {
                Some(&cnt) => {
                    mp.insert(nums1[i], cnt + 1);
                }
                None => {
                    mp.insert(nums1[i], 1);
                }
            }
        }
        //2. go through nums2
        for i in 0..nums2.len() {
            if let Some(&cnt) = mp.get(&nums2[i]) {
                res.push(nums2[i]);
                if cnt - 1 <= 0 {
                    mp.remove(&nums2[i]);
                } else {
                    mp.insert(nums2[i], cnt - 1);
                }
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
    fn test_350() {}
}
