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
}
