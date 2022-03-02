
use std::rc::Rc;

type Link<T: Ord> = Option<Rc<Node<T>>>;
pub struct List<T: Ord> {
    head: Link<T>,
}
struct Node<T: Ord> {
    elem: T,
    next: Link<T>,
}


impl<T: Ord> List<T: Ord> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn insert(&mut self, elem: T) {
        
        if let Some(h) = self.head {

            let mut 


        }
        else {
            self.head = Some(Rc::new(Node {
                elem: elem,
                next: None
            }));
        }
        
    }
}