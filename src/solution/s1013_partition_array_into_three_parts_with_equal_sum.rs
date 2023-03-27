/**
 * [1013] Partition Array Into Three Parts With Equal Sum
 *
 * Given an array of integers arr, return true if we can partition the array into three non-empty parts with equal sums.
 * Formally, we can partition the array if we can find indexes i + 1 < j with (arr[0] + arr[1] + ... + arr[i] == arr[i + 1] + arr[i + 2] + ... + arr[j - 1] == arr[j] + arr[j + 1] + ... + arr[arr.length - 1])
 *  
 * <strong class="example">Example 1:
 *
 * Input: arr = [0,2,1,-6,6,-7,9,1,2,0,1]
 * Output: true
 * Explanation: 0 + 2 + 1 = -6 + 6 - 7 + 9 + 1 = 2 + 0 + 1
 *
 * <strong class="example">Example 2:
 *
 * Input: arr = [0,2,1,-6,6,7,9,-1,2,0,1]
 * Output: false
 *
 * <strong class="example">Example 3:
 *
 * Input: arr = [3,3,6,5,-2,2,5,1,-9,4]
 * Output: true
 * Explanation: 3 + 3 = 6 = 5 - 2 + 2 + 5 + 1 - 9 + 4
 *
 *  
 * Constraints:
 *
 * 	3 <= arr.length <= 5 * 10^4
 * 	-10^4 <= arr[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/
// discuss: https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum: i32 = arr.iter().sum();
        if sum % 3 != 0 {
            return false;
        }
        let target = sum / 3;
        let mut part_sum = 0;
        let mut cnt = 0;
        for &n in arr.iter() {
            part_sum += n;
            if part_sum == target {
                cnt += 1;
                part_sum = 0;
            }
        }
        cnt >= 3
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1013() {}
}
