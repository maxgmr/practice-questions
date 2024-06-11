#![allow(unused)]

use practice_questions::Stack;

fn main() {
    println!("hello!");
}

// 3.2 Stack Min
// Design stack with O(1) time for push, pop, & min
#[derive(Default)]
pub struct StackMin<T>
where
    T: PartialOrd + Eq + Copy,
{
    stack: Vec<T>,
    min_stack: Vec<MinInfo<T>>,
}
impl<T> StackMin<T>
where
    T: PartialOrd + Eq + Copy,
{
    pub fn new() -> Self {
        StackMin {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item);

        if let Some(item) = self.stack.last() {
            if let Some(prev_min) = self.min() {
                if *item <= prev_min {
                    self.min_stack.push(MinInfo { min: *item });
                }
            } else {
                self.min_stack.push(MinInfo { min: *item });
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(curr_val) = self.stack.last() {
            if let Some(curr_min) = self.min() {
                if curr_min == *curr_val {
                    self.min_stack.pop();
                }
            }
        }
        self.stack.pop()
    }

    pub fn min(&self) -> Option<T> {
        self.min_stack.last().map(|min_info| min_info.min())
    }
}

struct MinInfo<T>
where
    T: Eq + PartialOrd + Copy,
{
    min: T,
}
impl<T> MinInfo<T>
where
    T: Eq + PartialOrd + Copy,
{
    fn min(&self) -> T {
        self.min
    }
}

// 3.3 Stack of Plates
// Implement a data structure that creates a new stack when the previous one exceeds capacity, but
// still acts like a regular stack otherwise.
#[derive(Default)]
pub struct SetOfStacks<T> {
    stack_set: Vec<Stack<T>>,
    max_size: usize,
}
impl<T> SetOfStacks<T> {
    pub fn new(max_size: usize) -> Self {
        if max_size < 1 {
            panic!("The stacks must hold at least one value!");
        }
        SetOfStacks {
            stack_set: Vec::new(),
            max_size,
        }
    }

    pub fn push(&mut self, item: T) {
        if let Some(mut last_stack) = self.stack_set.last_mut() {
            if last_stack.len() >= self.max_size {
                let mut new_stack: Stack<T> = Stack::new();
                new_stack.push(item);
                self.stack_set.push(new_stack);
            } else {
                last_stack.push(item);
            }
        } else {
            let mut new_stack: Stack<T> = Stack::new();
            new_stack.push(item);
            self.stack_set.push(new_stack);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut last_stack) = self.stack_set.last_mut() {
            if let Some(val) = last_stack.pop() {
                if last_stack.is_empty() {
                    self.stack_set.pop();
                }
                Some(val)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn pop_at(&mut self, index: usize) -> Option<T> {
        if let Some(mut chosen_stack) = self.stack_set.get_mut(index) {
            let output = chosen_stack.pop();
            if chosen_stack.is_empty() {
                self.stack_set.remove(index);
            }
            output
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.stack_set
            .iter()
            .fold(0_usize, |sum, stack| sum + stack.len())
    }

    pub fn num_stacks(&self) -> usize {
        self.stack_set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack_set.len() == 0
    }
}

// Q3.4 Queue via Stacks
// Implement a queue using two stacks.
#[derive(Debug, Default)]
struct TwoStackQueue<T> {
    sa: Stack<T>,
    sb: Stack<T>,
}
impl<T> TwoStackQueue<T> {
    fn new() -> Self {
        Self {
            sa: Stack::new(),
            sb: Stack::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.sa.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        if !self.sb.is_empty() {
            self.sb.pop()
        } else {
            while let Some(val) = self.sa.pop() {
                self.sb.push(val);
            }
            self.sb.pop()
        }
    }

    fn peek(&mut self) -> Option<&T> {
        if !self.sb.is_empty() {
            self.sb.peek()
        } else {
            while let Some(val) = self.sa.pop() {
                self.sb.push(val);
            }
            self.sb.peek()
        }
    }

    fn len(&self) -> usize {
        self.sa.len() + self.sb.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// Q3.5 Sort Stack
// Sort a stack such that the smallest items are on the top using only an additional temporary
// stack and not by copying the elements into any other data structure. The stack type supports
// push, pop, peek, and is_empty.
fn sort_stack<T>(mut stack: Stack<T>) -> Stack<T>
where
    T: PartialOrd + PartialEq,
{
    let mut helper_stack = Stack::<T>::new();
    while let Some(elem) = stack.pop() {
        while !helper_stack.is_empty() && *helper_stack.peek().unwrap() < elem {
            stack.push(helper_stack.pop().unwrap());
        }
        helper_stack.push(elem);
    }
    helper_stack
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn sm_basics() {
        let mut sm = StackMin::<i32>::new();
        assert_eq!(sm.min(), None);
        sm.push(2);
        assert_eq!(sm.min(), Some(2_i32));
        sm.push(1);
        assert_eq!(sm.min(), Some(1_i32));
        sm.push(3);
        assert_eq!(sm.min(), Some(1_i32));
        sm.push(1);
        assert_eq!(sm.min(), Some(1_i32));
        sm.pop();
        assert_eq!(sm.min(), Some(1_i32));
        sm.pop();
        assert_eq!(sm.min(), Some(1_i32));
        sm.pop();
        assert_eq!(sm.min(), Some(2_i32));
        sm.pop();
        assert_eq!(sm.min(), None);
    }

    #[test]
    fn sos_basics() {
        let mut sos = SetOfStacks::<u8>::new(3);
        assert!(sos.is_empty());
        assert_eq!(sos.len(), 0);

        sos.push(1);
        assert!(!sos.is_empty());
        assert_eq!(sos.len(), 1);

        assert_eq!(sos.pop(), Some(1));
        assert!(sos.is_empty());

        assert_eq!(sos.pop(), None);

        sos.push(1);
        assert_eq!(sos.len(), 1);
        assert_eq!(sos.num_stacks(), 1);

        sos.push(2);
        assert_eq!(sos.len(), 2);
        assert_eq!(sos.num_stacks(), 1);

        sos.push(3);
        assert_eq!(sos.len(), 3);
        assert_eq!(sos.num_stacks(), 1);

        sos.push(4);
        assert_eq!(sos.len(), 4);
        assert_eq!(sos.num_stacks(), 2);

        assert_eq!(sos.pop(), Some(4));
        assert_eq!(sos.len(), 3);
        assert_eq!(sos.num_stacks(), 1);

        sos.push(4);
        assert_eq!(sos.len(), 4);
        assert_eq!(sos.num_stacks(), 2);

        assert_eq!(sos.pop(), Some(4));
        assert_eq!(sos.len(), 3);
        assert_eq!(sos.num_stacks(), 1);

        assert_eq!(sos.pop(), Some(3));
        assert_eq!(sos.len(), 2);
        assert_eq!(sos.num_stacks(), 1);

        assert_eq!(sos.pop(), Some(2));
        assert_eq!(sos.len(), 1);
        assert_eq!(sos.num_stacks(), 1);

        assert_eq!(sos.pop(), Some(1));
        assert_eq!(sos.len(), 0);
        assert_eq!(sos.num_stacks(), 0);

        assert_eq!(sos.pop(), None);
    }

    #[test]
    fn sos_pop_at() {
        let mut sos = SetOfStacks::<char>::new(2);

        sos.push('h');
        sos.push('u');
        sos.push('l');
        sos.push('l');
        sos.push('o');

        assert_eq!(sos.pop_at(3), None);
        assert_eq!(sos.num_stacks(), 3);
        assert_eq!(sos.pop_at(1), Some('l'));
        assert_eq!(sos.pop_at(1), Some('l'));
        assert_eq!(sos.num_stacks(), 2);
        assert_eq!(sos.pop_at(1), Some('o'));
        assert_eq!(sos.num_stacks(), 1);
        assert_eq!(sos.pop_at(1), None);
        assert_eq!(sos.pop_at(0), Some('u'));
        assert_eq!(sos.num_stacks(), 1);
        assert_eq!(sos.pop_at(0), Some('h'));
        assert_eq!(sos.num_stacks(), 0);
        assert_eq!(sos.pop_at(0), None);
        assert_eq!(sos.pop_at(1), None);
        assert_eq!(sos.pop_at(2), None);
    }

    #[test]
    fn tsq_basics() {
        let mut tsq = TwoStackQueue::<i32>::new();

        assert!(tsq.is_empty());
        assert_eq!(tsq.len(), 0);
        assert_eq!(tsq.peek(), None);
        assert_eq!(tsq.pop(), None);

        tsq.push(1);
        assert_eq!(tsq.len(), 1);
        assert!(!tsq.is_empty());

        assert_eq!(tsq.peek(), Some(&1_i32));
        assert_eq!(tsq.len(), 1);
        assert!(!tsq.is_empty());

        tsq.push(2);
        assert_eq!(tsq.len(), 2);
        assert_eq!(tsq.peek(), Some(&1_i32));

        tsq.push(3);
        tsq.push(4);
        assert_eq!(tsq.len(), 4);
        assert_eq!(tsq.peek(), Some(&1_i32));

        assert_eq!(tsq.pop(), Some(1_i32));
        assert_eq!(tsq.len(), 3);
        assert_eq!(tsq.peek(), Some(&2_i32));

        assert_eq!(tsq.pop(), Some(2_i32));
        assert_eq!(tsq.pop(), Some(3_i32));
        assert_eq!(tsq.pop(), Some(4_i32));
        assert_eq!(tsq.pop(), None);
    }

    #[test]
    fn ss_empty() {
        let mut start = Stack::<i32>::new();
        assert_eq!(sort_stack(start).pop(), None);
    }

    #[test]
    fn ss_sorted() {
        let mut start = Stack::<u8>::new();

        start.push(3);
        start.push(2);
        start.push(1);

        start = sort_stack(start);

        let a = start.pop();
        let b = start.pop();
        let c = start.pop();
        let d = start.pop();

        assert_eq!(a, Some(1_u8));
        assert_eq!(b, Some(2_u8));
        assert_eq!(c, Some(3_u8));
        assert_eq!(d, None);
    }

    #[test]
    fn ss_unsorted() {
        let mut glossy = Stack::<char>::new();

        glossy.push('s');
        glossy.push('g');
        glossy.push('o');
        glossy.push('l');
        glossy.push('s');
        glossy.push('y');

        glossy = sort_stack(glossy);

        assert_eq!(glossy.pop(), Some('g'));
        assert_eq!(glossy.pop(), Some('l'));
        assert_eq!(glossy.pop(), Some('o'));
        assert_eq!(glossy.pop(), Some('s'));
        assert_eq!(glossy.pop(), Some('s'));
        assert_eq!(glossy.pop(), Some('y'));
        assert_eq!(glossy.pop(), None);
    }
}
