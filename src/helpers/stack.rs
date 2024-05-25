#[derive(Debug, Default)]
pub struct Stack<T> {
    stack: Vec<T>,
}
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            stack: Vec::with_capacity(capacity),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basics() {
        let mut stack: Stack<i32> = Stack::new();

        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());

        stack.push(1);
        assert_eq!(stack.len(), 1);
        assert!(!stack.is_empty());

        stack.push(2);
        assert_eq!(stack.len(), 2);
        assert!(!stack.is_empty());

        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.len(), 2);
        assert!(!stack.is_empty());

        stack.push(4);
        assert_eq!(stack.len(), 3);
        assert!(!stack.is_empty());
        assert_eq!(stack.peek(), Some(&4));

        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), Some(&2));

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek(), Some(&1));
        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());
    }
}
