/**
 * [228] Summary Ranges
 *
 * You are given a sorted unique integer array nums.
 * A range [a,b] is the set of all integers from a to b (inclusive).
 * Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.
 * Each range [a,b] in the list should be output as:
 *
 * 	"a->b" if a != b
 * 	"a" if a == b
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [0,1,2,4,5,7]
 * Output: ["0->2","4->5","7"]
 * Explanation: The ranges are:
 * [0,2] --> "0->2"
 * [4,5] --> "4->5"
 * [7,7] --> "7"
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [0,2,3,4,6,8,9]
 * Output: ["0","2->4","6","8->9"]
 * Explanation: The ranges are:
 * [0,0] --> "0"
 * [2,4] --> "2->4"
 * [6,6] --> "6"
 * [8,9] --> "8->9"
 *
 *  
 * Constraints:
 *
 * 	0 <= nums.length <= 20
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	All the values of nums are unique.
 * 	nums is sorted in ascending order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/summary-ranges/
// discuss: https://leetcode.com/problems/summary-ranges/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        if nums.len() == 0 {
            return ret;
        }
        let mut start = 0;
        let mut end = 0;
        while start < nums.len() {
            end = start;
            while end + 1 < nums.len() && nums[end + 1] == nums[end] + 1 {
                end += 1;
            }
            let mut s = nums[start].to_string();
            if start != end {
                s = s + "->" + &nums[end].to_string();
                start = end + 1;
            } else {
                start += 1;
            }
            ret.push(s);
        }
        ret
    }

    pub fn summary_ranges_1(nums: Vec<i32>) -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        if nums.len() == 0 {
            return ret;
        }
        let mut start = nums[0];
        let mut end = nums[0];
        for i in 1..nums.len() {
            if nums[i] == (nums[i - 1] + 1) {
                end = nums[i];
            } else {
                let mut s = start.to_string();
                if start != end {
                    s = s + "->" + &end.to_string();
                }
                ret.push(s);
                start = dbg!(nums[i]);
                end = dbg!(nums[i]);
            }
        }
        let mut s = start.to_string();
        if start != end {
            s = s + "->" + &end.to_string();
        }
        ret.push(s);
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_228_1() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );
    }
    #[test]
    fn test_228_2() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );
    }
    #[test]
    fn test_228_3() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 4, 5, 6, 8, 10]),
            vec!["0", "2", "4->6", "8", "10"]
        );
    }
    #[test]
    fn test_228_4() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 4, 5]),
            vec!["0->1", "4->5"]
        );
    }
}
