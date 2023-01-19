// Problem description: https://leetcode.com/problems/lru-cache/

#![allow(dead_code)]

use std::{
    alloc::{alloc, Layout},
    collections::HashMap,
    ptr::null_mut,
};

// Doubly-linked list
struct DLLNode {
    key: i32,
    val: i32,
    prev: *mut DLLNode,
    next: *mut DLLNode,
}

struct LRUCache {
    first: *mut DLLNode,
    last: *mut DLLNode,

    map: HashMap<i32, *mut DLLNode>,
}

const INVALID_KEY: i32 = -1;

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let layout = Layout::new::<DLLNode>();
        let mut head: *mut DLLNode = null_mut();
        let mut tail: *mut DLLNode = null_mut();

        unsafe {
            for _ in 0..capacity {
                let new_head = alloc(layout) as *mut DLLNode;

                if new_head.is_null() {
                    panic!("DLLNode allocation failed");
                }

                if tail.is_null() {
                    tail = new_head;
                }

                (*new_head).next = head;
                (*new_head).val = 0;
                (*new_head).key = INVALID_KEY;

                if !head.is_null() {
                    (*head).prev = new_head;
                }

                head = new_head;
            }

            (*head).prev = null_mut();
        }

        Self {
            first: head,
            last: tail,
            map: HashMap::new(),
        }
    }

    unsafe fn move_to_front(&mut self, node: *mut DLLNode) {
        if (*node).prev.is_null() {
            return;
        }

        if (*node).next.is_null() {
            self.last = (*node).prev;
            (*(*node).prev).next = null_mut();
        } else {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
        }

        (*node).next = self.first;
        (*node).prev = null_mut();
        (*self.first).prev = node;
        self.first = node;
    }

    fn get(&mut self, key: i32) -> i32 {
        // Look up the key in the hash table
        // If it isn't there, return -1
        // Else, remove the given DLLNode from the linked list and put it at the front
        // Return the value in that DLLNode
        match self.map.get(&key) {
            Some(&node_ptr) => unsafe {
                self.move_to_front(node_ptr);
                assert_eq!((*node_ptr).key, key);
                (*node_ptr).val
            },
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // Look up the key in the hash table
        // If it's there, take the DLLNode pointer
        //  - reassign its value
        //  - move it to the front of the list
        // If it isn't there:
        //  - take the last node off of the linked list
        //  - unmap the node's corresponding key, if it's valid
        //  - map the new key to the new node
        //  - set the key/value in the new node
        //  - move it to the front of the list

        match self.map.get(&key) {
            Some(&node_ptr) => unsafe {
                (*node_ptr).val = value;
                assert_eq!((*node_ptr).key, key);
                self.move_to_front(node_ptr);
            },
            None => {
                let node_ptr = self.last;
                unsafe {
                    if (*node_ptr).key != INVALID_KEY {
                        self.map.remove(&(*node_ptr).key);
                    }

                    (*node_ptr).key = key;
                    (*node_ptr).val = value;
                    self.map.insert(key, node_ptr);

                    self.move_to_front(node_ptr);
                }
            }
        }
    }
}

#[test]
fn do_test() {
    let mut cache = LRUCache::new(2);

    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);

    let mut cache = LRUCache::new(1);
    cache.put(2, 1);
    assert_eq!(cache.get(2), 1);
    cache.put(3, 2);
    assert_eq!(cache.get(2), -1);
    assert_eq!(cache.get(3), 2);
}
