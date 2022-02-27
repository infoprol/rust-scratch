
use std::rc::Rc;

pub struct List<T> { head: Link<T>, }
type Link<T> = Option<Rc<Node<T>>>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node {
            elem: elem,
            next: self.head.clone(),
        }))}
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|n| n.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem )
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}


//iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_deref();
            &n.elem
        })
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(n) = head {
            if let Ok(mut n) = Rc::try_unwrap(n) {
                head = n.next.take();
            }
            else { break; }
        }
    }
}



//tests
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let ll = List::new();
        assert_eq!(ll.head(), None);

        let ll = ll.prepend(1).prepend(2).prepend(3);
        assert_eq!(ll.head(), Some(&3));

        let ll = ll.tail();
        assert_eq!(ll.head(), Some(&2));

        let ll = ll.tail();
        assert_eq!(ll.head(), Some(&1));

        assert_eq!(ll.tail().head(), None);
    }

    fn iter() {
        let ll = List::new().prepend(1).prepend(2).prepend(3);
        let mut iter = ll.iter();

        let mut k = 3;
        for i in iter {
            assert_eq!(k, *i);
            k -= 1;
        }
    }

}