/*
    Going off the list exercises, going to try a simple BST.
*/


use std::rc::Rc;

pub struct Bst<T: Ord> { root: Link<T>, }
//type Node<T> = Option<Rc<Node<T>>>;
type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Node {
            elem: elem,
            left: None,
            right: None
        }
    }
}

impl<T: Ord> Bst<T> {
    pub fn new() -> Self {
        Bst { root: None }
    }

    pub fn insert(&mut self, elem: T)  {

        let mut curr = self.root.as_ref();


        match &self.root {
            None => {
                self.root = Some(Rc::new(Node::new(elem)));
            },
            Some(r) => {

                let mut curr = r;
                //recurse til we hit None
                if elem < curr.elem {

                }


            }
        }
        

        ;
    }

}