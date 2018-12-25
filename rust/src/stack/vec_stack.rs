use stack::Stack;

/**
Trivial implementation of a stack by dispatching to Vector
*/
pub struct VecStack {
    vec: Vec<i32>

}

impl VecStack {
    fn new() -> Self {
        VecStack {
            vec: vec![],
        }
    }
}

impl Stack for VecStack {
    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn size(&self) -> usize {
        self.vec.len()
    }

    fn push(&mut self, item: i32) {
        self.vec.push(item);
    }

    fn pop(&mut self) -> Option<i32> {
        self.vec.pop()
    }

    // Returns a reference since stack still owns peeked value
    fn peek(&self) -> Option<&i32> {
        self.vec.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        VecStack::new();
    }

    #[test]
    fn init_is_empty() {
        let stack = VecStack::new();
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn push() {
        let mut stack = VecStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn pop_from_empty() {
        let mut stack = VecStack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn pop_gets_last_item() {
        let mut stack = VecStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
    }

    #[test]
    fn pop_whole_stack() {
        let mut stack = VecStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek() {
        let mut stack = VecStack::new();
        assert_eq!(stack.peek(), None);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.peek(), Some(&3));
    }
}

