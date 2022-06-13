
use std::fmt::Display;
use std::mem;

#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    root: Option<Box<Node<T>>>,
    len: i32,
}

impl<T: Display + Eq> LinkedList<T> {
    pub fn new(val: T) -> Self {
        Self {
            root: Some(Box::new(Node {
                val: val,
                next: None,
            })),
            len: 1,
        }
    }

    pub fn len(&self) -> i32 {
        return self.len;
    }

    pub fn find(&self, target: T) -> Option<i32> {
        let mut cur = &self.root;
        let mut idx = 0;
        loop {
            match &cur {
                None => return None,
                Some(node) => {
                    if node.val == target {
                        return Some(idx);
                    }
                    cur = &node.next;
                }
            }
            idx += 1
        }
    }

    pub fn print(&self) {
        if self.len() == 0 {
            print!("[EMPTY]\n");
            return;
        }
        let mut cur = &self.root;
        loop {
            match &cur {
                None => break,
                Some(v) => {
                    print!("{} ", v.val);
                    cur = &v.next;
                }
            }
        }
        print!("\n")
    }

    fn insert_next(node: &mut Box<Node<T>>, val: T) {
        let n = Box::new(Node {
            val: val,
            next: node.next.take(),
        });
        node.next = Some(n)
    }

    pub fn insert(&mut self, val: T, pos: i32) {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(Node {
                    val: val,
                    next: None,
                }))
            }
            Some(cur) => {
                let mut it = cur;
                let mut cnt = 0;
                loop {
                    let is_last: bool = { it.next.is_none() };
                    if cnt == pos || is_last {
                        LinkedList::insert_next(it, val);
                        self.len += 1;
                        return;
                    }
                    match &mut it.next {
                        None => break,
                        Some(next) => it = next,
                    }
                    cnt += 1;
                }
            }
        }
    }

    pub fn remove(&mut self, pos: i32) -> Option<T> {
        if pos == 0 {
            match mem::replace(&mut self.root, None) {
                None => return None,
                Some(n) => {
                    self.root = n.next;
                    self.len -= 1;
                    return Some(n.val);
                }
            }
        }

        match &mut self.root {
            None => return None,
            Some(node) => {
                let mut it = node;
                let mut cnt = 1;
                loop {
                    if cnt == pos {
                        match mem::replace(&mut it.next, None) {
                            None => return None,
                            Some(n) => {
                                it.next = n.next;
                                self.len -= 1;
                                return Some(n.val);
                            }
                        }
                    }
                    match &mut it.next {
                        None => break,
                        Some(n) => it = n,
                    }
                    cnt += 1;
                }
            }
        }
        return None;
    }
}
