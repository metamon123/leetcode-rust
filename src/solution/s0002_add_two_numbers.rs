/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *
 * Example:
 *
 *
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/add-two-numbers/
// discuss: https://leetcode.com/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy_head = Some(Box::new(ListNode::new(0))); // Box should be declared with 'mut' if it's content is going to be changed
        let mut tail = &mut dummy_head;
        let mut quotient = 0;
        loop {
            let val1 =
                if let Some(box1) = l1 {
                    l1 = (*box1).next;
                    (*box1).val
                } else {
                    0
                };
            let val2 =
                if let Some(box2) = l2 {
                    l2 = (*box2).next;
                    (*box2).val
                } else {
                    0
                };

            let sum = val1 + val2 + quotient;
            let remainder = sum % 10;
            quotient = sum / 10;

            // Make new ListNode
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(remainder)));
            tail = &mut tail.as_mut().unwrap().next;

            // Determine whether loop should be continued or not
            match (&l1, &l2) {
                (None, None) => {
                    if quotient <= 0 {
                        // No need to continue
                        break dummy_head.unwrap().next;
                    }
                }
                _ => ()
            };


        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}
