use self::{tree_node::TreeNode, tree_node_wrapper::TreeNodeWrapper};
use std::{cell::RefCell, fmt::Display, ops::Deref, rc::Rc};

mod tree_node;
mod tree_node_wrapper;

pub struct AvlTree<'a, T: PartialOrd + PartialEq + Display> {
    root: Option<Rc<RefCell<TreeNode<'a, T>>>>,
    height: usize,
}

impl<'a, T: PartialOrd + PartialEq + Display> AvlTree<'a, T> {
    pub fn new() -> Self {
        AvlTree {
            root: None,
            height: 0,
        }
    }
    pub fn iter(&'a self) -> TreeAsIter<'a, T> {
        let curr = if let Some(root) = self.root.as_ref() {
            Some(Rc::clone(root))
        } else {
            None
        };

        TreeAsIter {
            parents: Vec::with_capacity(self.height),
            curr,
        }
    }
}

pub struct TreeAsIter<'a, T: PartialOrd + PartialEq + Display> {
    parents: Vec<Rc<RefCell<TreeNode<'a, T>>>>,
    curr: Option<Rc<RefCell<TreeNode<'a, T>>>>,
}

impl<'a, T: PartialOrd + PartialEq + Display> Iterator for TreeAsIter<'a, T> {
    type Item = TreeNodeWrapper<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(traverse) = if let Some(item) = self.curr.as_ref() {
            Some(Rc::clone(item))
        } else {
            None
        } {
            let item = Rc::clone(&traverse);
            self.parents.push(traverse);
            self.curr = if let Some(left) = item.deref().borrow().left.as_ref() {
                Some(Rc::clone(left))
            } else {
                None
            };
        }

        match !self.parents.is_empty() {
            true => {
                let item = self.parents.pop().unwrap();
                self.curr = if let Some(right) = item.deref().borrow().right.as_ref() {
                    Some(Rc::clone(right))
                } else {
                    None
                };
                Some(TreeNodeWrapper { guard: item })
            }
            false => None,
        }

        // let mut traverse = if let Some(item) = self.curr.as_ref() {
        //     Some(Rc::clone(item))
        // } else {
        //     None
        // };

        // let moved: bool;
        // self.where_to = if let WhereTo::Right = self.where_to {
        //     if unwind {
        //
        //     }
        //     (traverse, moved) = get_right_helper(traverse);
        //     match moved {
        //         true => {
        //             if let Some(last) = self.parents.last() {
        //                 if last.deref().borrow().get() == traverse.unwrap().deref().borrow().get() {
        //                     self.parents.pop();
        //                     traverse = get_parent_helper(traverse);
        //                     traverse = get_parent_helper(traverse);
        //                 } else {
        //                     self.parents.push(Rc::clone(traverse.as_ref().unwrap()));
        //                     traverse = get_left_helper(traverse);
        //                 }
        //             }
        //         }
        //         false => {
        //             traverse = get_parent_helper(traverse);
        //         }
        //     }
        // } else {
        //     if let Some(item) = traverse.as_ref() {
        //         self.parents.push(Rc::clone(item))
        //     }
        //     traverse = get_left_helper(traverse);
        //     WhereTo::Right
        // };
        //
        // self.curr = if let Some(traverse) = traverse.as_ref() {
        //     Some(Rc::clone(traverse))
        // } else {
        //     None
        // };
    }
}

// fn get_parent_helper<T: PartialEq>(
//     traverse: Option<Rc<RefCell<TreeNode<T>>>>,
// ) -> Option<Rc<RefCell<TreeNode<T>>>> {
//     if let Some(item) = traverse.as_ref() {
//         if item.deref().borrow().parent.is_some() {
//             Weak::upgrade(item.deref().borrow().parent.as_ref().unwrap())
//         } else {
//             None
//         }
//     } else {
//         None
//     }
// }
//
// fn get_right_helper<T: PartialEq>(
//     traverse: Option<Rc<RefCell<TreeNode<T>>>>,
// ) -> (Option<Rc<RefCell<TreeNode<T>>>>, bool) {
//     if let Some(item) = traverse.as_ref() {
//         if item.deref().borrow().right.is_some() {
//             (
//                 Some(Rc::clone(item.deref().borrow().right.as_ref().unwrap())),
//                 true,
//             )
//         } else {
//             (traverse, false)
//         }
//     } else {
//         (None, false)
//     }
// }
//
// fn get_left_helper<T: PartialEq>(
//     mut traverse: Option<Rc<RefCell<TreeNode<T>>>>,
// ) -> Option<Rc<RefCell<TreeNode<T>>>> {
//     while traverse.is_some() && traverse.as_ref().unwrap().deref().borrow().left.is_some() {
//         traverse = if let Some(item) = traverse.as_ref() {
//             Some(Rc::clone(item.deref().borrow().left.as_ref().unwrap()))
//         } else {
//             None
//         };
//     }
//     traverse
// }
