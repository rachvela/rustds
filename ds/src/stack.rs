use std::fmt::Display;
use std::mem;

#[derive(Debug)]
struct Node<T> {
  val: T,
  next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
  head: Option<Box<Node<T>>>,
  len: i32,
}

impl<T: Display> Stack<T> {
  pub fn new() -> Self {
    Stack { head: None, len: 0 }
  }

  pub fn len(&self) -> i32 {
    self.len
  }

  pub fn pop(&mut self) -> Option<T> {
    match mem::replace(&mut self.head, None) {
      None => None,
      Some(node) => {
        self.len -= 1;
        self.head = node.next;
        Some(node.val)
      }
    }
  }

  pub fn top(&self) -> Option<&T> {
    match &self.head {
      None => None,
      Some(node) => Some(&node.val),
    }
  }

  pub fn push(&mut self, val: T) {
    self.len += 1;
    match mem::replace(&mut self.head, None) {
      None => self.head = Some(Box::new(Node { val, next: None })),
      Some(node) => {
        self.head = Some(Box::new(Node {
          val,
          next: Some(node),
        }))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn basic() {
    let mut list = Stack::<String>::new();
    assert_eq!(0, list.len());
    assert_eq!(None, list.top());
    list.push(String::from("val1"));
    assert_eq!(Some(&String::from("val1")), list.top());
    assert_eq!(Some(&String::from("val1")), list.top());
    list.push(String::from("val2"));
    assert_eq!(Some(&String::from("val2")), list.top());

    assert_eq!(Some(String::from("val2")), list.pop());
    assert_eq!(1, list.len());
    assert_eq!(Some(String::from("val1")), list.pop());
    assert_eq!(0, list.len());
    assert_eq!(None, list.pop());

  }
}
