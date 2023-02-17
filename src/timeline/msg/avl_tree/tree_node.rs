use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct TreeNode<'a, T>
where
    T: PartialOrd,
{
    item: &'a T,
    pub left: Option<Rc<RefCell<TreeNode<'a, T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<'a, T>>>>,
    pub parent: Option<Weak<RefCell<TreeNode<'a, T>>>>,
}

impl<'a, T: PartialOrd> TreeNode<'a, T> {
    pub fn new(data: &'a T) -> Self {
        TreeNode {
            item: data,
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn get(&self) -> &T {
        &self.item
    }
}
