/**
 * [1323] Maximum 69 Number
 *
 * You are given a positive integer num consisting only of digits 6 and 9.
 * Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).
 *  
 * <strong class="example">Example 1:
 *
 * Input: num = 9669
 * Output: 9969
 * Explanation:
 * Changing the first digit results in 6669.
 * Changing the second digit results in 9969.
 * Changing the third digit results in 9699.
 * Changing the fourth digit results in 9666.
 * The maximum number is 9969.
 *
 * <strong class="example">Example 2:
 *
 * Input: num = 9996
 * Output: 9999
 * Explanation: Changing the last digit 6 to 9 results in the maximum number.
 *
 * <strong class="example">Example 3:
 *
 * Input: num = 9999
 * Output: 9999
 * Explanation: It is better not to apply any change.
 *
 *  
 * Constraints:
 *
 * 	1 <= num <= 10^4
 * 	num consists of only 6 and 9 digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-69-number/
// discuss: https://leetcode.com/problems/maximum-69-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum69_number(mut num: i32) -> i32 {
        let mut arr: Vec<i32> = vec![];
        while num > 0 {
            arr.push(num % 10);
            num /= 10;
        }
        for i in (0..arr.len()).rev() {
            if arr[i] == 6 {
                arr[i] = 9;
                break;
            }
        }
        arr.iter().rev().fold(0, |mut acc, n| {
            acc = acc * 10 + n;
            return acc;
        })
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1323() {}
}
