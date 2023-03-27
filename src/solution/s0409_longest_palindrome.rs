/**
 * [409] Longest Palindrome
 *
 * Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.
 * Letters are case sensitive, for example, "Aa" is not considered a palindrome here.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "abccccdd"
 * Output: 7
 * Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "a"
 * Output: 1
 * Explanation: The longest palindrome that can be built is "a", whose length is 1.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 2000
 * 	s consists of lowercase and/or uppercase English letters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindrome/
// discuss: https://leetcode.com/problems/longest-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
     pub fn longest_palindrome_magaic(s: String) -> i32 {
        (s.chars()
            .fold(HashMap::new(), |mut acc, ch| {
                acc.entry(ch).and_modify(|count| *count += 1).or_insert(1);
                acc
            })
            .iter()
            .map(|(_, &count)| count & !1)
            .sum::<i32>()
            + 1)
        .min(s.len() as i32)
    }
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for key in s.chars() {
            let var = counts.entry(key).or_insert(0);
            *var += 1;
        }
        let mut found_odd = false;
        let sum_of_even_parts: i32 = counts.into_iter().fold(0, |acc, (_, x)| {
            found_odd = found_odd || x % 2 == 1;
            acc + (x / 2) * 2
        });
        sum_of_even_parts + if found_odd { 1 } else { 0 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_409() {}
}
