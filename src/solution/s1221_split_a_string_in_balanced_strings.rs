/**
 * [1221] Split a String in Balanced Strings
 *
 * Balanced strings are those that have an equal quantity of 'L' and 'R' characters.
 * Given a balanced string s, split it into some number of substrings such that:
 *
 * 	Each substring is balanced.
 *
 * Return the maximum number of balanced strings you can obtain.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "RLRRLLRLRL"
 * Output: 4
 * Explanation: s can be split into "RL", "RRLL", "RL", "RL", each substring contains same number of 'L' and 'R'.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "RLRRRLLRLL"
 * Output: 2
 * Explanation: s can be split into "RL", "RRRLLRLL", each substring contains same number of 'L' and 'R'.
 * Note that s cannot be split into "RL", "RR", "RL", "LR", "LL", because the 2^nd and 5^th substrings are not balanced.
 * <strong class="example">Example 3:
 *
 * Input: s = "LLLLRRRR"
 * Output: 1
 * Explanation: s can be split into "LLLLRRRR".
 *
 *  
 * Constraints:
 *
 * 	2 <= s.length <= 1000
 * 	s[i] is either 'L' or 'R'.
 * 	s is a balanced string.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-a-string-in-balanced-strings/
// discuss: https://leetcode.com/problems/split-a-string-in-balanced-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut l_cnt = 0;
        let mut r_cnt = 0;
        let mut res = 0;
        for ch in s.chars() {
            match ch {
                'L' => {
                    l_cnt += 1;
                }
                'R' => {
                    r_cnt += 1;
                }
                _ => unreachable!(),
            }
            if l_cnt == r_cnt {
                res += 1;
                l_cnt = 0;
                r_cnt = 0;
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
    fn test_1221() {}
}
