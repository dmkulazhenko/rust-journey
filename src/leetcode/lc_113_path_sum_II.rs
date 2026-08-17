// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn dfs(v: Ref<TreeNode>, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, mut path_sum: i32, target_sum: i32) {
        path.push(v.val);
        path_sum += v.val;
        if v.left.is_none() && v.right.is_none() && path_sum == target_sum {
            ans.push(path.clone());
        }

        if let Some(left) = &v.left {
            Self::dfs(left.borrow(), path, ans, path_sum, target_sum);
        }
        if let Some(right) = &v.right {
            Self::dfs(right.borrow(), path, ans, path_sum, target_sum);
        }

        path.pop();
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        if root.is_none() { return vec![]; }
        let root = root.unwrap();

        let mut path = Vec::new();
        let mut ans = Vec::new();
        Self::dfs(root.borrow(), &mut path, &mut ans, 0, target_sum);

        ans
    }
}

fn main() {}
