
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::cmp::Ord;

type Link = Rc<RefCell<TNode>>;

#[derive(Debug)]
pub struct TNode {
    val: i32,
    left: Option<Link>,
    right: Option<Link>,
}


#[derive(Debug)]
pub struct Bst {
    root: Option<Link>,
}   


impl Bst {
    pub fn new_link(val: i32) -> Link {
        Rc::new(RefCell::new(TNode {
            val: val,
            left: None,
            right: None,
        }))
    }
    pub fn new() -> Self {
        Bst { root: None }
    }

    pub fn insert(&mut self, val: i32) {
        let new_node = Bst::new_link(val);

        match self.root.take() {
            Nothing => {
                self.root = Some(new_node);
            },
            Some(root) => {
                let mut curr = root.clone();

                //restore root
                self.root = Some(root);

                loop {

                    match val.cmp(&curr.borrow().val) {
                        Ordering::Less => {
                                
                            match curr.borrow().left.as_ref().map(|x| x.clone()) {
                                None => {
                                    curr.borrow_mut().left = Some(Bst::new_link(val));
                                    break;
                                },
                                Some(n) => {
                                    
                                    let m = n.clone();  
                                    curr.borrow_mut().left = Some(n);
                                    curr.borrow().left.as_ref().map(|x| x.clone());
                                },
                            }
                        },
                        Ordering::Equal | Ordering::Greater => {}
                        ,
                    }


                } //loop

            } //Some(root)
        } //match

    } //insert

} //impl