use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Queue<T> {
    queue: VecDeque<T>,
}
impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push_back(item)
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basics() {
        let mut queue: Queue<i32> = Queue::new();

        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());

        queue.enqueue(1);
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());

        queue.enqueue(2);
        assert_eq!(queue.len(), 2);
        assert!(!queue.is_empty());

        queue.enqueue(3);
        assert_eq!(queue.len(), 3);
        assert!(!queue.is_empty());

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.len(), 2);
        assert!(!queue.is_empty());

        queue.enqueue(4);
        assert_eq!(queue.len(), 3);
        assert!(!queue.is_empty());
        assert_eq!(queue.peek(), Some(&2));

        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.peek(), Some(&3));

        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek(), Some(&4));
        assert!(!queue.is_empty());

        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());

        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
    }
}
