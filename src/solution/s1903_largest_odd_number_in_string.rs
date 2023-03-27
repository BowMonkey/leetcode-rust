/**
 * [1903] Largest Odd Number in String
 *
 * You are given a string num, representing a large integer. Return the largest-valued odd integer (as a string) that is a non-empty substring of num, or an empty string "" if no odd integer exists.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * <strong class="example">Example 1:
 *
 * Input: num = "52"
 * Output: "5"
 * Explanation: The only non-empty substrings are "5", "2", and "52". "5" is the only odd number.
 *
 * <strong class="example">Example 2:
 *
 * Input: num = "4206"
 * Output: ""
 * Explanation: There are no odd numbers in "4206".
 *
 * <strong class="example">Example 3:
 *
 * Input: num = "35427"
 * Output: "35427"
 * Explanation: "35427" is already an odd number.
 *
 *  
 * Constraints:
 *
 * 	1 <= num.length <= 10^5
 * 	num only consists of digits and does not contain any leading zeros.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-odd-number-in-string/
// discuss: https://leetcode.com/problems/largest-odd-number-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        for (i, ch) in num.chars().rev().enumerate() {
            let n = ch.to_digit(10).unwrap();
            if n % 2 == 1 {
                return num.clone().drain(0..num.len() - i).collect();
            }
        }
        String::new()
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1903() {}
}
