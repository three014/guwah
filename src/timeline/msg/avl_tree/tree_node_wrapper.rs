use std::{
    cell::{Ref, RefCell},
    fmt::Display,
    ops::Deref,
    rc::Rc,
};

use super::tree_node::TreeNode;

pub struct TreeNodeWrapper<'a, T: PartialOrd + PartialEq + Display> {
    pub guard: Rc<RefCell<TreeNode<'a, T>>>,
}

impl<'a, T: PartialOrd + PartialEq + Display> TreeNodeWrapper<'a, T> {
    pub fn get(&self) -> impl Display + Deref<Target = T> + '_ {
        Ref::map(self.guard.borrow(), |item| item.get())
    }
}
