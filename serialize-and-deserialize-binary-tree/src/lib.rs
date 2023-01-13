#![allow(dead_code)]

use shared::*;

struct Codec;

use std::cell::RefCell;
use std::rc::Rc;

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
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

    fn deserialize(&self, mut data: String) -> Option<Rc<RefCell<TreeNode>>> {
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
}

#[test]
fn do_test() {
    let codec = Codec::new();

    let input = "[1,2,3,null,null,4,5]".to_string();
    let deser = codec.deserialize(input.clone());
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
    let ser = codec.serialize(deser);
    assert_eq!(input, ser);

    let input = "[]".to_string();
    let deser = codec.deserialize(input.clone());
    let expected = None;

    assert_eq!(deser, expected);
    let ser = codec.serialize(deser);
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

    assert_eq!(codec.deserialize(codec.serialize(tree.clone())), tree);
}
