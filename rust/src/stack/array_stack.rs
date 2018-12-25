use super::Stack;

pub struct ArrayStack {
    capacity: usize,
    top: usize,
    arr: Box<[isize]>,

}

impl ArrayStack {
    fn new(capacity: usize) -> Self {

        ArrayStack {
            capacity: capacity,
            top: 0,
            arr: vec![0; capacity].into_boxed_slice(),
        }
    }
}

impl Stack for ArrayStack {
    fn is_empty(&self) -> bool {
        unimplemented!()
    }

    fn size(&self) -> u32 {
        unimplemented!()
    }

    fn push(&mut self, item: i32) {
        unimplemented!()
    }

    fn pop(&mut self) -> Option<i32> {
        unimplemented!()
    }

    fn peek(&self) -> Option<i32> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        ArrayStack::new(10);
    }

    #[test]
    fn init_is_empty() {
        let stack = ArrayStack::new(10);
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn push() {
        let mut stack = ArrayStack::new(10);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn pop_from_empty() {
        let mut stack = ArrayStack::new(10);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn pop_gets_last_item() {
        let mut stack = ArrayStack::new(10);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
    }

    #[test]
    fn pop_whole_stack() {
        let mut stack = ArrayStack::new(10);
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
    fn push_past_capacity() {
        let mut stack = ArrayStack::new(2);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.pop(), None);
    }
}

