#![allow(unused)]

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

struct Stack<T> {
    stack: Vec<T>,
}
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.len() == 0
    }
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
}
