/*
    Going off the list exercises, going to try a simple BST.
*/


use std::rc::Rc;
use std::cell::RefCell;

pub struct Bst<T: Ord> { root: Link<T>, }
//type Node<T> = Option<Rc<Node<T>>>;
type Link<T> = Option<Box<Node<T>>>;

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

type TLink = Option<RefCell<Rc<TNode>>>;
struct TNode {
    elem: i32,
    left: TLink,
    right: TLink,
}

pub fn bst_scratch() {

}







impl<T: Ord> Bst<T> {
    pub fn new() -> Self {
        Bst { root: None }
    }

    fn mk_link(elem: T) -> Link<T> {
        Some(Box::new(Node::new(elem)))
    }
/*

    fn do_insert(mut root: Link<T>, elem: T) -> Link<T> {
        if let Some(&mut n) = root.as_deref_mut() {
            
            
            Some(Box::new(n))
        }
        else {
            Bst::mk_link(elem)
        }



    }

*/
    pub fn insert(&mut self, elem: T)  {

        let mut curr = self.root.as_ref();
        
/*
        match &self.root {
            None => {
                self.root = Bst::mk_link(elem)
            },
            Some(r) => {

                let mut curr = r;
                //recurse til we hit None
                if elem < curr.elem {
                    //insert into left subtree, if it is null
                    match &curr.left {
                        None => {
                            curr.left = Bst::mk_link(elem);
                        },
                        Some(l) => {
                            curr = &l;
                        }
                    }
                }


            }
        }
        

        ;
        */
    }

}