/**
 * [605] Can Place Flowers
 *
 * You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.
 * Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.
 *  
 * <strong class="example">Example 1:
 * Input: flowerbed = [1,0,0,0,1], n = 1
 * Output: true
 * <strong class="example">Example 2:
 * Input: flowerbed = [1,0,0,0,1], n = 2
 * Output: false
 *  
 * Constraints:
 * 
 * 	1 <= flowerbed.length <= 2 * 10^4
 * 	flowerbed[i] is 0 or 1.
 * 	There are no two adjacent flowers in flowerbed.
 * 	0 <= n <= flowerbed.length
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/can-place-flowers/
// discuss: https://leetcode.com/problems/can-place-flowers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut newbed=vec![0];
        newbed.append(&mut flowerbed);
        newbed.push(0);
        let mut count=0;
        for i in 1..newbed.len()-1{
            if newbed[i-1]==0&&newbed[i]==0&&newbed[i+1]==0{
                newbed[i]=1;
                count+=1;
            }
        }
        count>=n
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_605() {
    }
}
