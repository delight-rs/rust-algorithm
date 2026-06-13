//! 108. 将有序数组转换为二叉搜索树
//! https://leetcode.cn/problems/convert-sorted-array-to-binary-search-tree/description/

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {

        TreeNode {
            val: val,
            left: None,
            right: None,
        }
    }

}

struct Solution {}

impl Solution {
    /**
     * nums: 有序数据
     */
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();

        if len <= 0 {
            return None;
        }

        Some(Self::create_tree(&nums, 0, len - 1))
    }

    fn create_tree(nums: &Vec<i32>, i: usize, j: usize) -> Rc<RefCell<TreeNode>> {
        if i == j {
            return Self::create_tree_node(nums[i]);
        }

        let mid = (i + j) >> 1;
        let root = Self::create_tree_node(nums[mid]);

        if i < mid {
            root.borrow_mut().left = Some(Self::create_tree(nums, i, mid - 1));
        }

        if j > mid {
            root.borrow_mut().right = Some(Self::create_tree(nums, mid + 1, j));
        }

        return root;
    }

    fn create_tree_node(v: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(v)))
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Solution;
    use super::TreeNode;

    #[test]
    fn test_sorted_array_to_bst() {
        let nums = [-10, -3, 0, 5, 9].to_vec();

        let tree = Solution::sorted_array_to_bst(nums.clone());
        let mut expected = Vec::new();

        traverse_tree(tree.unwrap(), &mut expected);

        assert_eq!(nums, expected)
    }

    fn traverse_tree(header: Rc<RefCell<TreeNode>>, nums: &mut Vec<i32>) {
        let val = header.borrow().val;
        let left = header.borrow().left.clone();
        let right = header.borrow().right.clone();

        if left.is_some() {
            traverse_tree(left.clone().unwrap(), nums);
        }

        nums.push(val);

        if right.is_some() {
            traverse_tree(right.unwrap(), nums);
        }
    }
}
