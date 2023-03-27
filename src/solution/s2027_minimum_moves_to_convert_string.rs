/**
 * [2027] Minimum Moves to Convert String
 *
 * You are given a string s consisting of n characters which are either 'X' or 'O'.
 * A move is defined as selecting three consecutive characters of s and converting them to 'O'. Note that if a move is applied to the character 'O', it will stay the same.
 * Return the minimum number of moves required so that all the characters of s are converted to 'O'.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "XXX"
 * Output: 1
 * Explanation: <u>XXX</u> -> OOO
 * We select all the 3 characters and convert them in one move.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "XXOX"
 * Output: 2
 * Explanation: <u>XXO</u>X -> O<u>OOX</u> -> OOOO
 * We select the first 3 characters in the first move, and convert them to 'O'.
 * Then we select the last 3 characters and convert them so that the final string contains all 'O's.
 * <strong class="example">Example 3:
 *
 * Input: s = "OOOO"
 * Output: 0
 * Explanation: There are no 'X's in s to convert.
 *
 *  
 * Constraints:
 *
 * 	3 <= s.length <= 1000
 * 	s[i] is either 'X' or 'O'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-moves-to-convert-string/
// discuss: https://leetcode.com/problems/minimum-moves-to-convert-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let (mut count, mut moved) = (0, 0);
        for ch in s.chars() {
            match ch {
                'X' if moved == 0 => {
                    count += 1;
                    moved = 2;
                }
                _ if moved != 0 => {
                    moved -= 1;
                }
                _ => {},
            }
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2027() {}
}