#![allow(dead_code)]

use std::{cmp::Ordering, collections::BinaryHeap};

struct Solution;

#[derive(Eq)]
struct FutureTask {
    pub id: usize,
    pub start: i32,
    pub duration: i32,
}

impl PartialOrd for FutureTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FutureTask {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

impl Ord for FutureTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // The task that starts earliest is the most important
        match self.start {
            x if x < other.start => Ordering::Greater,
            x if x == other.start => Ordering::Equal,
            _ => Ordering::Less,
        }
    }
}

// A pending task is one that *can* start, but hasn't yet.
#[derive(Eq)]
struct PendingTask {
    pub id: usize,
    pub duration: i32,
}

impl PartialOrd for PendingTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PendingTask {
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration
    }
}

impl Ord for PendingTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // The task with shortest duration is most important
        match self.duration {
            x if x < other.duration => Ordering::Greater,
            x if x > other.duration => Ordering::Less,
            _ => match self.id {
                x if x < other.id => Ordering::Greater,
                x if x == other.id => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
    }
}

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut future_tasks = tasks
            .into_iter()
            .enumerate()
            .map(|(index, task)| FutureTask {
                id: index,
                start: task[0],
                duration: task[1],
            })
            .collect::<BinaryHeap<FutureTask>>();

        let mut pending_tasks = BinaryHeap::<PendingTask>::new();

        let mut task_order = Vec::<i32>::new();

        let mut now = 0;

        while !(future_tasks.is_empty() && pending_tasks.is_empty()) {
            // If any future tasks have passed their 'start' time, add them to pending_tasks.
            while let Some(ft) = future_tasks.peek() {
                if ft.start <= now {
                    pending_tasks.push(PendingTask {
                        id: ft.id,
                        duration: ft.duration,
                    });
                    future_tasks.pop();
                } else {
                    break;
                }
            }

            // If there are no pending tasks, wait until the start time fo the next future task, in
            // order. Otherwise, do a task.
            if pending_tasks.is_empty() {
                if let Some(ft) = future_tasks.peek() {
                    now = ft.start;
                }
            } else {
                let task = pending_tasks.pop().unwrap();
                now += task.duration;
                task_order.push(task.id as i32);
            }
        }

        task_order
    }
}

#[test]
fn test1() {
    let tasks = vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]];

    let expected = vec![0, 2, 3, 1];

    assert_eq!(Solution::get_order(tasks), expected);
}

#[test]
fn test2() {
    let tasks = vec![vec![7, 10], vec![7, 12], vec![7, 5], vec![7, 4], vec![7, 2]];

    let expected = vec![4, 3, 2, 0, 1];

    assert_eq!(Solution::get_order(tasks), expected);
}
