use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub  struct TreeNode {
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

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::HashMap;
        #[allow(clippy::too_many_arguments)]
        fn build_tree_helper(
            preorder: &[i32],
            inorder: &[i32],
            inorder_indices: &HashMap<i32, usize>,
            preorder_indices: &HashMap<i32, usize>,
            pre_start_index: usize,
            pre_end_index: usize,
            in_start_index: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if pre_end_index <= pre_start_index {
                return None;
            }
            let in_center_val = preorder[pre_start_index];
            let in_center_index = *inorder_indices.get(&in_center_val).unwrap();
            let left_len = in_center_index - in_start_index;
            let left = build_tree_helper(
                preorder,
                inorder,
                inorder_indices,
                preorder_indices,
                pre_start_index + 1,
                pre_start_index + 1 + left_len,
                in_start_index,
            );
            let right = build_tree_helper(
                preorder,
                inorder,
                inorder_indices,
                preorder_indices,
                pre_start_index + 1 + left_len,
                pre_end_index,
                in_center_index + 1,
            );
            Some(Rc::new(RefCell::new(TreeNode {
                val: in_center_val,
                left,
                right,
            })))
        }
        let inorder_indices =
            HashMap::<i32, usize>::from_iter(inorder.iter().enumerate().map(|(i, v)| (*v, i)));
        let preorder_indices =
            HashMap::<i32, usize>::from_iter(preorder.iter().enumerate().map(|(i, v)| (*v, i)));
        build_tree_helper(
            &preorder,
            &inorder,
            &inorder_indices,
            &preorder_indices,
            0,
            preorder.len(),
            0,
        )
    }

    pub fn build_lc_tree(tree_vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = tree_vec.len();
        if len == 0 {
            return None;
        }
        let root = tree_vec[0].map(|v| Rc::new(RefCell::new(TreeNode::new(v))));
        {
            let mut working = VecDeque::new();
            working.push_back(root.clone());
            let mut is_left = true;
            for i in 1..len {
                while let Some(curr) = working.pop_front() {
                    if let Some(curr_ref) = curr.clone() {
                        let mut curr_node = curr_ref.borrow_mut();
                        let new_node = tree_vec[i].map(|v| Rc::new(RefCell::new(TreeNode::new(v))));
                        if is_left {
                            curr_node.left = new_node.clone();
                            is_left = false;
                            working.push_front(curr);
                        } else {
                            curr_node.right = new_node.clone();
                            is_left = true;
                        }
                        working.push_back(new_node);
                        break;
                    }
                }
            }
        }
        root.clone()
    }
}

#[macro_export]
macro_rules! tree_node {
    ($elem: expr, $left: expr, $right: expr) => {
        Some(::std::rc::Rc::new(::std::cell::RefCell::new(
            crate::solutions::util::trees::TreeNode {
                val: $elem,
                left: $left,
                right: $right,
            },
        )))
    };
}

#[macro_export]
macro_rules! tree_leaf {
    ($elem: expr) => {
        tree_node!($elem, None, None)
    };
}

#[macro_export]
macro_rules! lc_tree_vec {
  () => {
    Vec::<Option<i32>>::new()
  };
  ( $x:expr ) => {
    {
      let mut temp_vec = Vec::<Option<i32>>::new();
      temp_vec.push( Some( $x ) );
      temp_vec
    }
  };
  ( null ,  $($rest: tt),* ) => {
    {
      let mut temp_vec = Vec::<Option<i32>>::new();
      temp_vec.push(None);
      temp_vec.extend( ( crate::lc_tree_vec![ $($rest),*  ] ).iter());
      temp_vec
    }
  };
  ( $x:expr ,  $($rest:tt),*  ) => {
    {
      let mut temp_vec = Vec::<Option<i32>>::new();
      temp_vec.push( Some( $x ) );
      temp_vec.extend( ( crate::lc_tree_vec![ $($rest),* ]  ).iter());
      temp_vec
    }
  };
}

#[macro_export]
macro_rules! lc_tree {
  ( $($rest: tt),* ) => {
    {
      crate::solutions::util::trees::TreeNode::build_lc_tree( crate::lc_tree_vec![ $($rest),* ] )
    }
  };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lc_tree, lc_tree_vec, tree_leaf, tree_node};

    #[test]
    fn test_lc_tree_vec_macro_1() {
        let i = lc_tree_vec![2, null, 2];
        let o = vec![Some(2), None, Some(2)];
        assert_eq!(i, o);
    }

    #[test]
    fn test_build_lc_tree_1() {
        let i = vec![Some(1), Some(2), Some(3), Some(4), None, Some(5)];
        let o = tree_node!(
            1,
            tree_node!(2, tree_leaf!(4), None),
            tree_node!(3, tree_leaf!(5), None)
        );
        assert_eq!(TreeNode::build_lc_tree(i), o);
    }

    #[test]
    fn test_lc_tree_macro_1() {
        let i = lc_tree![1, 2, 3, 4, null, 5];
        let o = tree_node!(
            1,
            tree_node!(2, tree_leaf!(4), None),
            tree_node!(3, tree_leaf!(5), None)
        );
        assert_eq!(i, o);
    }
}
