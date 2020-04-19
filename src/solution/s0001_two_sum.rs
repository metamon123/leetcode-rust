/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they add up to a specific target.
 *
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 *
 * Example:
 *
 *
 * Given nums = [2, 7, 11, 15], target = 9,
 *
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum/
// discuss: https://leetcode.com/problems/two-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_idx_map: HashMap<i32, Vec<usize>> = HashMap::with_capacity(nums.len());
        for (idx, num) in nums.iter().enumerate() {
            let rest = target - num;
            match num_idx_map.get(&rest) {
                Some(idx_lst) => {
                    let mut ans = vec![idx_lst[0] as i32, idx as i32];
                    ans.sort();
                    return ans;
                }
                None => {
                    let idx_lst = num_idx_map.entry(*num).or_insert(vec![]);
                    idx_lst.push(idx);
                }
            };
        }
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![1,2,3,4,5], 8), [2, 4]);
        assert_eq!(Solution::two_sum(vec![2,2,3,4,5], 4), [0, 1]);
    }
}
