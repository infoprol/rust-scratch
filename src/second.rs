

type Link<T> = Option<Box<Node<T>>>;
pub struct List<T> {
    head: Link<T>,
}
struct Node<T> {
    elem: T,
    next: Link<T>,
}    


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| {
            &n.elem
        })
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| {
            &mut n.elem
        })
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter<'a>(&'a self) -> Iter<'a,T> {
        Iter { next: self.head.as_deref() }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_deref();
            & n.elem
        })
    }
}

pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();
        while let Some(mut boxed_node) = curr_link {
            curr_link = boxed_node.next.take();
        }
    }
}



#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut ll = List::new();
        assert_eq!(ll.pop(), None);

        ll.push(1);
        ll.push(2);
        ll.push(3);

        assert_eq!(ll.pop(), Some(3));
        assert_eq!(ll.pop(), Some(2));

        ll.push(4);
        ll.push(5);

        assert_eq!(ll.pop(), Some(5));
        assert_eq!(ll.pop(), Some(4));
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), None);

    }
    #[test]
    fn peek() {
        let mut ll = List::new();
        assert_eq!(ll.peek(), None);
        assert_eq!(ll.peek_mut(), None);

        for k in 1..4 {
            ll.push(k);
        }

        assert_eq!(ll.peek(), Some(&3));
        assert_eq!(ll.peek_mut(), Some(&mut 3));

        ll.peek_mut().map(|val| {
            *val = 42;
        });

        assert_eq!(ll.peek(), Some(&42));
        assert_eq!(ll.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut ll = List::new();
        assert_eq!(ll.pop(), None);

        for j in 1..4 {
            ll.push(j);
        }
        let mut iter = ll.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut ll = List::new();
        for j in 1..4 { ll.push(j); }

        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}


