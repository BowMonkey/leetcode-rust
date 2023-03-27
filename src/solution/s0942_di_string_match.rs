/**
 * [942] DI String Match
 *
 * A permutation perm of n + 1 integers of all the integers in the range [0, n] can be represented as a string s of length n where:
 * 
 * 	s[i] == 'I' if perm[i] < perm[i + 1], and
 * 	s[i] == 'D' if perm[i] > perm[i + 1].
 * 
 * Given a string s, reconstruct the permutation perm and return it. If there are multiple valid permutations perm, return any of them.
 *  
 * <strong class="example">Example 1:
 * Input: s = "IDID"
 * Output: [0,4,1,3,2]
 * <strong class="example">Example 2:
 * Input: s = "III"
 * Output: [0,1,2,3]
 * <strong class="example">Example 3:
 * Input: s = "DDI"
 * Output: [3,2,0,1]
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^5
 * 	s[i] is either 'I' or 'D'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/di-string-match/
// discuss: https://leetcode.com/problems/di-string-match/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut res:Vec<i32>=vec![];
        let mut mx=s.len() as i32;
        let mut mi=0 ;
        for ch in s.chars(){
            match ch{
                'I'=>{ res.push(mi); mi+=1;},
                'D'=>{ res.push(mx); mx-=1;},
                _=>unreachable!()
            }
        }
        res.push(mi);
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_942_1() {
        assert_eq!(Solution::di_string_match( "IDID".to_string()), vec![0,4,1,3,2]);
    }
    #[test]
    fn test_942_2() {
        assert_eq!(Solution::di_string_match( "III".to_string()), vec![0,1,2,3]);
    }
    #[test]
    fn test_942_3() {
        assert_eq!(Solution::di_string_match(  "DDI".to_string()), vec![3,2,0,1]);
    }
}
