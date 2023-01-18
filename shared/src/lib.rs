#![allow(dead_code)]

pub use std::cell::RefCell;
pub use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    pub const fn new(val: i32) -> Self {
        Self { next: None, val }
    }

    pub fn from_slice(slice: &[i32]) -> Option<Box<Self>> {
        if slice.is_empty() {
            None
        } else {
            Some(Box::new(Self {
                val: slice[0],
                next: Self::from_slice(&slice[1..]),
            }))
        }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn tree_serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    if root.is_none() {
        return "[]".into();
    }

    let mut level: Vec<Rc<RefCell<TreeNode>>> = vec![root.unwrap()];
    let mut next_level: Vec<Rc<RefCell<TreeNode>>> = vec![];
    let mut values: Vec<Option<i32>> = vec![Some(level[0].borrow().val)];
    let mut ser = "[".to_string();

    while !level.is_empty() {
        for node in level {
            if let Some(ref left) = node.borrow().left {
                next_level.push(left.clone());
                values.push(Some(left.borrow().val));
            } else {
                values.push(None);
            }

            if let Some(ref right) = node.borrow().right {
                next_level.push(right.clone());
                values.push(Some(right.borrow().val));
            } else {
                values.push(None);
            }
        }

        level = next_level;
        next_level = vec![];
    }

    // Last few values should be None, but we won't record those
    while values.last().unwrap().is_none() {
        values.pop();
    }

    for (i, v) in values.iter().enumerate() {
        match v {
            Some(value) => ser += &format!("{value}"),
            None => ser += "null",
        };

        if i != values.len() - 1 {
            ser.push(',');
        }
    }

    ser + "]"
}

pub fn tree_deserialize(data: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let mut data = data.to_string();

    // remove the []
    data.pop();
    data.remove(0);

    if data.is_empty() {
        return None;
    }

    let mut values: Vec<Option<i32>> = vec![];
    for v in data.split(',') {
        values.push(if v == "null" {
            None
        } else {
            Some(v.parse::<i32>().unwrap())
        });
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));

    let mut level: Vec<Rc<RefCell<TreeNode>>> = vec![root.clone()];
    let mut next_level = vec![];
    let mut index = 1;

    while !level.is_empty() {
        for node in level {
            if index >= values.len() {
                break;
            }

            if let Some(value) = values[index] {
                let new_node = Rc::new(RefCell::new(TreeNode::new(value)));
                next_level.push(new_node.clone());
                node.borrow_mut().left = Some(new_node.clone());
            }

            index += 1;
            if index >= values.len() {
                break;
            }

            if let Some(value) = values[index] {
                let new_node = Rc::new(RefCell::new(TreeNode::new(value)));
                next_level.push(new_node.clone());
                node.borrow_mut().right = Some(new_node.clone());
            }

            index += 1;
        }

        level = next_level;
        next_level = vec![];
    }

    Some(root)
}

#[test]
fn test_serialize_deserialize() {
    let input = "[1,2,3,null,null,4,5]".to_string();
    let deser = tree_deserialize(&input);
    let expected = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
    })));

    assert_eq!(deser, expected);
    let ser = tree_serialize(deser);
    assert_eq!(input, ser);

    let input = "[]".to_string();
    let deser = tree_deserialize(&input);
    let expected = None;

    assert_eq!(deser, expected);
    let ser = tree_serialize(deser);
    assert_eq!(input, ser);

    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
    })));

    assert_eq!(tree_deserialize(&tree_serialize(tree.clone())), tree);
}
