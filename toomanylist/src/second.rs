//!
//! a ok stack
//! push,pop,iter,iter_mut,into_iter,peek,peek_mut
//! 

use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a mut self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(), // !
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            // !
            self.head = node.next;
            node.elem
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        println!("droplist");

        let mut cur_link = self.head.take(); // !
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}




pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));

        // !
        let p = list.peek_mut();
        *p.unwrap() = 100;
        assert_eq!(list.peek(), Some(&100));

        // !
        let _ = list.peek_mut().map(|value| *value = 200);
        assert_eq!(list.peek(), Some(&200));

        for i in list.into_iter() {
            println!("{i}");
        }
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        for i in list.into_iter() {
            println!("{i}");
        }
    }
    #[test]
    fn test_iter() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        for i in list.iter() {
            println!("{i}");
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        for i in list.iter_mut() {
            *i *= 2;
        }

        for i in list.iter() {
            println!("{i}");
        }
    }
}
