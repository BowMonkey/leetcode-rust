/**
 * [680] Valid Palindrome II
 *
 * Given a string s, return true if the s can be palindrome after deleting at most one character from it.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "aba"
 * Output: true
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "abca"
 * Output: true
 * Explanation: You could delete the character 'c'.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "abc"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-palindrome-ii/
// discuss: https://leetcode.com/problems/valid-palindrome-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        for i in 0..n / 2 {
            if s[i] != s[n - i - 1] {
                let l = &s[i..n - i - 1];
                let r = &s[i + 1..n - i];
                return Self::helper(l) || Self::helper(r);
            }
        }
        true
    }
    fn helper(s: &[u8]) -> bool {
        let n = s.len();
        (0..n / 2).all(|i| s[i] == s[n - i - 1])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::thread::ScopedJoinHandle;

    use super::*;

    #[test]
    fn test_680_1() {
        assert_eq!(Solution::valid_palindrome("abas".to_string()), true);
    }
    #[test]
    fn test_680_2() {
        assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
    }
    #[test]
    fn test_680_3() {
        assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
    }
    #[test]
    fn test_680_4() {
        assert_eq!(Solution::valid_palindrome("a".to_string()), true);
    }
    #[test]
    fn test_680_5() {
        assert_eq!(Solution::valid_palindrome("abba".to_string()), true);
    }
    #[test]
    fn test_680_6() {
        assert_eq!(
            Solution::valid_palindrome(
                "lcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupucul".to_string()
            ),
            true
        );
    }
}
