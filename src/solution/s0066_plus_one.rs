/**
 * [66] Plus One
 *
 * You are given a large integer represented as an integer array digits, where each digits[i] is the i^th digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
 * Increment the large integer by one and return the resulting array of digits.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: digits = [1,2,3]
 * Output: [1,2,4]
 * Explanation: The array represents the integer 123.
 * Incrementing by one gives 123 + 1 = 124.
 * Thus, the result should be [1,2,4].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: digits = [4,3,2,1]
 * Output: [4,3,2,2]
 * Explanation: The array represents the integer 4321.
 * Incrementing by one gives 4321 + 1 = 4322.
 * Thus, the result should be [4,3,2,2].
 * 
 * <strong class="example">Example 3:
 * 
 * Input: digits = [9]
 * Output: [1,0]
 * Explanation: The array represents the integer 9.
 * Incrementing by one gives 9 + 1 = 10.
 * Thus, the result should be [1,0].
 * 
 *  
 * Constraints:
 * 
 * 	1 <= digits.length <= 100
 * 	0 <= digits[i] <= 9
 * 	digits does not contain any leading 0's.
 * 
 */

/*

 思路：
 1. 倒序遍历数组，使用flag标志记录进位。如果进位保持到遍历结束，则数组头插入1
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/plus-one/
// discuss: https://leetcode.com/problems/plus-one/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut flag = 1;
        for n in digits.iter_mut().rev() {
            let val = *n + flag;
            *n = val%10;
            flag = val/10;
        }
        if flag==1{ digits.insert(0,1);}
        digits
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_66() {
    }
}
