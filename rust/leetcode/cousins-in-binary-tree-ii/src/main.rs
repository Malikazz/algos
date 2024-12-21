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
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;

pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    root
}

pub fn main() {
    let mut root = Rc::new(RefCell::new(TreeNode::new(5)));
    &root.borrow_mut().left = Rc::new(RefCell::new(TreeNode::new(4))); 
    replace_value_in_tree(Some(root));
}

pub fn make_tree_from_list(mut nums: Vec<i32>, mut root: Rc<RefCell<TreeNode>>) {
    let mut size: usize = 2;
    loop {
        for (count, item) in nums.windows(size).enumerate() {
            let mut current_parent: &Option<Rc<RefCell<TreeNode>>>;
            if count % 2 != 0 {
                current_parent = &root.borrow().left;
            } else {
                current_parent = &root.borrow().right;
            }
            current_parent = &Some(Rc::new(RefCell::new(TreeNode::new(item[0]))));
        }
        nums = nums[size..].to_vec();
        size = size * 2;
    }
}
