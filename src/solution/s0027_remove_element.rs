/**
 * [27] Remove Element
 *
 * Given an integer array nums and an integer val, remove all occurrences of val in nums <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a>. The relative order of the elements may be changed.
 * Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
 * Return k after placing the final result in the first k slots of nums.
 * Do not allocate extra space for another array. You must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 * Custom Judge:
 * The judge will test your solution with the following code:
 * 
 * int[] nums = [...]; // Input array
 * int val = ...; // Value to remove
 * int[] expectedNums = [...]; // The expected answer with correct length.
 *                             // It is sorted with no values equaling val.
 * int k = removeElement(nums, val); // Calls your implementation
 * assert k == expectedNums.length;
 * sort(nums, 0, k); // Sort the first k elements of nums
 * for (int i = 0; i < actualLength; i++) {
 *     assert nums[i] == expectedNums[i];
 * }
 * 
 * If all assertions pass, then your solution will be accepted.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [3,2,2,3], val = 3
 * Output: 2, nums = [2,2,_,_]
 * Explanation: Your function should return k = 2, with the first two elements of nums being 2.
 * It does not matter what you leave beyond the returned k (hence they are underscores).
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [0,1,2,2,3,0,4,2], val = 2
 * Output: 5, nums = [0,1,4,0,3,_,_,_]
 * Explanation: Your function should return k = 5, with the first five elements of nums containing 0, 0, 1, 3, and 4.
 * Note that the five elements can be returned in any order.
 * It does not matter what you leave beyond the returned k (hence they are underscores).
 * 
 *  
 * Constraints:
 * 
 * 	0 <= nums.length <= 100
 * 	0 <= nums[i] <= 50
 * 	0 <= val <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-element/
// discuss: https://leetcode.com/problems/remove-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx:usize=0; // 非val数组的下一个位置
        for i in 0..nums.len(){
            if nums[i] != val{
                nums[idx] = nums[i];
                idx += 1;
            }
        }
        idx as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27_1() {
        let mut nums=vec![1];
        let val:i32 =1;
        assert_eq!(Solution::remove_element(&mut nums, val), 0);
        let res=vec![];
        assert_eq!(nums, res);
    }
    #[test]
    fn test_27_2() {
        let mut nums=vec![3,2,2,3];
        let val:i32 =3;
        assert_eq!(Solution::remove_element(&mut nums, val), 0);
        let res=vec![2,2];
        assert_eq!(nums, res);
    }
    #[test]
    fn test_27_3() {
        let mut nums=vec![0,1,2,2,3,0,4,2];
        let val:i32 =2;
        assert_eq!(Solution::remove_element(&mut nums, val), 0);
        let res=vec![0,1,3,0,4];
        assert_eq!(nums, res);
    }
}
