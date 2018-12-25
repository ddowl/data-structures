use super::Stack;

/**
This stack is implemented with a dynamically resizing array that's manually managed.
*/
pub struct ArrayStack {
    capacity: usize,
    top: usize,
    arr: Box<[i32]>,

}

impl ArrayStack {
    fn new(capacity: usize) -> Self {
        ArrayStack {
            capacity: capacity,
            top: 0,
            // initializing with vec might be cheating. I'm wasting time filling up the array with zeros. How would I allocate this manually?
            arr: vec![0; capacity].into_boxed_slice(),
        }
    }

    fn resize(&mut self, new_len: usize) {
        // Wasting time filling up with zeros
        let mut new_allocation = vec![0; new_len].into_boxed_slice();
        new_allocation[..self.capacity].clone_from_slice(&self.arr);
        println!("{:?}", new_allocation);
        self.arr = new_allocation;
        self.capacity = new_len;
    }
}

impl Stack for ArrayStack {
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn size(&self) -> usize {
        self.top
    }

    fn push(&mut self, item: i32) {
        if self.top == self.capacity {
            self.resize(self.capacity * 2);
        }
        self.arr[self.top] = item;
        self.top += 1;
    }

    fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.top -= 1;
            let item = self.arr[self.top];
            Some(item)
        }
    }

    fn peek(&self) -> Option<&i32> {
        if self.is_empty() {
            None
        } else {
            Some(&self.arr[self.top - 1])
        }
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
    fn peek() {
        let mut stack = ArrayStack::new(10);
        assert_eq!(stack.peek(), None);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.peek(), Some(&3));
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

