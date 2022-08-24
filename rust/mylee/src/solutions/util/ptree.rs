use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32, parent: Option<Rc<RefCell<TreeNode>>>) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
            parent,
        }
    }
}

pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap(), None))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(
                v,
                Some(parent.clone()),
            ))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(
                    v,
                    Some(parent.clone()),
                ))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}

#[macro_export]
macro_rules! ptree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            crate::solutions::util::ptree::to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(ptree![$($e),*])};
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    pub fn test_macro() {
        let _empty_tree: Option<i32> = tree![];
        tree![1,];
        tree![1, 1];
        tree![1, null, 1];
        tree![1, 1, 1];
        tree![1, 1, 1, 1];
        tree![1, null, 1, 1];
        tree![-1, null, -1, -1, null, -1];
    }
}
