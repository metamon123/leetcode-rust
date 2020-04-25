/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * <div>
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">"abcabcbb"</span>
 * Output: <span id="example-output-1">3
 * Explanation:</span> The answer is "abc", with the length of 3.
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">"bbbbb"</span>
 * Output: <span id="example-output-2">1
 * </span><span id="example-output-1">Explanation: T</span>he answer is "b", with the length of 1.
 *
 *
 * <div>
 * Example 3:
 *
 *
 * Input: <span id="example-input-3-1">"pwwkew"</span>
 * Output: <span id="example-output-3">3
 * </span><span id="example-output-1">Explanation: </span>The answer is "wke", with the length of 3.
 *              Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 * </div>
 * </div>
 * </div>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::{HashSet, HashMap};
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = 0;
        let mut max_substr_len = 0;
        let mut occurrence_map = HashSet::with_capacity(s.len());
        let mut chr_idx_map = HashMap::with_capacity(s.len());

        // Don't consider Unicode Strings (related with graphemes)

        for (idx, chr) in s.chars().enumerate() {
            // Make substring that ends with (idx, chr)

            // (0) String preprocessing to make further O(1) string indexing
            chr_idx_map.insert(idx, chr);

            // (1) If s[idx] exists in substring s[start:idx-1], keep removing first char of s[start:idx-1] until there is no s[idx]
            while occurrence_map.contains(&chr) {
                assert!(start < idx);

                let fst_chr = chr_idx_map.get(&start).unwrap();
                occurrence_map.remove(fst_chr);
                start += 1;
            }

            // (2) Now substring s[start:idx] doesn't contain repeating character. Update occurrence_map.
            occurrence_map.insert(chr);

            // (3) update max_len
            let new_substr_len = idx - start + 1;
            max_substr_len = if new_substr_len >= max_substr_len { new_substr_len } else { max_substr_len };
        }

        max_substr_len as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_longest_substring(String::from("abcabcbb")), 3);
        assert_eq!(Solution::length_of_longest_substring(String::from("bbbbb")), 1);
        assert_eq!(Solution::length_of_longest_substring(String::from("pwwkew")), 3);
        assert_eq!(Solution::length_of_longest_substring(String::from("a")), 1);
    }
}
