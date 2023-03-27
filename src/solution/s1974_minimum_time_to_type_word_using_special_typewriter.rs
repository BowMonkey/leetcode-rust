/**
 * [1974] Minimum Time to Type Word Using Special Typewriter
 *
 * There is a special typewriter with lowercase English letters 'a' to 'z' arranged in a circle with a pointer. A character can only be typed if the pointer is pointing to that character. The pointer is initially pointing to the character 'a'.
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/31/chart.jpg" style="width: 530px; height: 410px;" />
 * Each second, you may perform one of the following operations:
 *
 * 	Move the pointer one character counterclockwise or clockwise.
 * 	Type the character the pointer is currently on.
 *
 * Given a string word, return the minimum number of seconds to type out the characters in word.
 *  
 * <strong class="example">Example 1:
 *
 * Input: word = "abc"
 * Output: 5
 * Explanation:
 * The characters are printed as follows:
 * - Type the character 'a' in 1 second since the pointer is initially on 'a'.
 * - Move the pointer clockwise to 'b' in 1 second.
 * - Type the character 'b' in 1 second.
 * - Move the pointer clockwise to 'c' in 1 second.
 * - Type the character 'c' in 1 second.
 *
 * <strong class="example">Example 2:
 *
 * Input: word = "bza"
 * Output: 7
 * Explanation:
 * The characters are printed as follows:
 * - Move the pointer clockwise to 'b' in 1 second.
 * - Type the character 'b' in 1 second.
 * - Move the pointer counterclockwise to 'z' in 2 seconds.
 * - Type the character 'z' in 1 second.
 * - Move the pointer clockwise to 'a' in 1 second.
 * - Type the character 'a' in 1 second.
 *
 * <strong class="example">Example 3:
 *
 * Input: word = "zjpc"
 * Output: 34
 * Explanation:
 * The characters are printed as follows:
 * - Move the pointer counterclockwise to 'z' in 1 second.
 * - Type the character 'z' in 1 second.
 * - Move the pointer clockwise to 'j' in 10 seconds.
 * - Type the character 'j' in 1 second.
 * - Move the pointer clockwise to 'p' in 6 seconds.
 * - Type the character 'p' in 1 second.
 * - Move the pointer counterclockwise to 'c' in 13 seconds.
 * - Type the character 'c' in 1 second.
 *
 *  
 * Constraints:
 *
 * 	1 <= word.length <= 100
 * 	word consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-time-to-type-word-using-special-typewriter/
// discuss: https://leetcode.com/problems/minimum-time-to-type-word-using-special-typewriter/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cmp::min;
impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut res = 0;
        let mut pre = 'a';
        for ch in word.chars() {
            res += min(
                (ch as i32 - pre as i32).abs(),
                (26 - (ch as i32 - pre as i32).abs()),
            );
            pre = ch;
        }
        res + word.chars().count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1974_2() {
        assert_eq!(Solution::min_time_to_type("bza".to_string()), 7);
    }
    #[test]
    fn test_1974_1() {
        assert_eq!(Solution::min_time_to_type("abc".to_string()), 5);
    }
}
