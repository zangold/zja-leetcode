// https://leetcode.com/problems/min-stack/

#![allow(dead_code)]

struct MinStack {
    // second integer in the pair is the minimum of the elements below this one in the stack.
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        let new_min = if self.stack.is_empty() {
            val
        } else {
            val.min(self.get_min())
        };

        self.stack.push((val, new_min))
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

#[test]
fn do_test() {
    let mut ms = MinStack::new();
    ms.push(-2);
    ms.push(0);
    ms.push(-3);
    assert_eq!(ms.get_min(), -3);
    ms.pop();
    assert_eq!(ms.top(), 0);
    assert_eq!(ms.get_min(), -2);
}
