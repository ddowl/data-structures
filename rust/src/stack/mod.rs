pub trait Stack {
//    fn new() -> Self;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn push(&mut self, item: i32);
    fn pop(&mut self) -> Option<i32>;
    fn peek(&self) -> Option<&i32>;
}

mod vec_stack;
mod array_stack;
