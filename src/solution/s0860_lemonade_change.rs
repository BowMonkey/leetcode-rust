/**
 * [860] Lemonade Change
 *
 * At a lemonade stand, each lemonade costs $5. Customers are standing in a queue to buy from you and order one at a time (in the order specified by bills). Each customer will only buy one lemonade and pay with either a $5, $10, or $20 bill. You must provide the correct change to each customer so that the net transaction is that the customer pays $5.
 * Note that you do not have any change in hand at first.
 * Given an integer array bills where bills[i] is the bill the i^th customer pays, return true if you can provide every customer with the correct change, or false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: bills = [5,5,5,10,20]
 * Output: true
 * Explanation:
 * From the first 3 customers, we collect three $5 bills in order.
 * From the fourth customer, we collect a $10 bill and give back a $5.
 * From the fifth customer, we give a $10 bill and a $5 bill.
 * Since all customers got correct change, we output true.
 *
 * <strong class="example">Example 2:
 *
 * Input: bills = [5,5,10,10,20]
 * Output: false
 * Explanation:
 * From the first two customers in order, we collect two $5 bills.
 * For the next two customers in order, we collect a $10 bill and give back a $5 bill.
 * For the last customer, we can not give the change of $15 back because we only have two $10 bills.
 * Since not every customer received the correct change, the answer is false.
 *
 *  
 * Constraints:
 *
 * 	1 <= bills.length <= 10^5
 * 	bills[i] is either 5, 10, or 20.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/lemonade-change/
// discuss: https://leetcode.com/problems/lemonade-change/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut money = vec![0; 5];
        const FIVE: usize = 5 / 5;
        const TEN: usize = 10 / 5;
        const TWENTRY: usize = 20 / 5;
        for (idx, &i) in bills.iter().enumerate() {
            let mut change = i;
            money[i as usize / 5] += 1;
            change -= 5;
            while change > 10 && money[TEN] > 0 {
                money[TEN] -= 1;
                change -= 10;
            }
            while change > 0 && money[FIVE] > 0 {
                money[FIVE] -= 1;
                change -= 5;
            }
            if change > 0 {
                return false;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::thread::ScopedJoinHandle;

    use super::*;

    #[test]
    fn test_860_1() {
        assert_eq!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]), true);
    }
    #[test]
    fn test_860_2() {
        assert_eq!(Solution::lemonade_change(vec![5, 5, 10, 10, 20]), false);
    }
    #[test]
    fn test_860_3() {
        assert_eq!(
            Solution::lemonade_change(vec![
                5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5
            ]),
            true
        );
    }
}
