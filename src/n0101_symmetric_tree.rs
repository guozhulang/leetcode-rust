/**
 * [101] Symmetric Tree
 *
 * Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).
 * 
 * 
 * For example, this binary tree [1,2,2,3,4,4,3] is symmetric:
 * 
 *     1
 *    / \
 *   2   2
 *  / \ / \
 * 3  4 4  3
 * 
 * 
 * 
 * But the following [1,2,2,null,3,null,3]  is not:<br />
 * 
 *     1
 *    / \
 *   2   2
 *    \   \
 *    3    3
 * 
 * 
 * 
 * 
 * Note:<br />
 * Bonus points if you could solve it both recursively and iteratively.
 * 
 */
pub struct Solution {}
use super::util::tree::{TreeNode, to_tree};

// submission codes start here

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::symmetric_helper(root.as_ref().and_then(|v| v.borrow().left.clone()),
                                   root.as_ref().and_then(|v| v.borrow().right.clone()))
    }

    fn symmetric_helper(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (Some(left), Some(right)) => {
                left.borrow().val == right.borrow().val &&
                    Solution::symmetric_helper(left.borrow().left.clone(), right.borrow().right.clone()) &&
                    Solution::symmetric_helper(left.borrow().right.clone(), right.borrow().left.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_101() {
        assert_eq!(Solution::is_symmetric(tree![1,2,2,3,4,4,3]), true);
        assert_eq!(Solution::is_symmetric(tree![1,2,2,null,3,null,3]), false);
        assert_eq!(Solution::is_symmetric(tree![]), true);
    }
}
