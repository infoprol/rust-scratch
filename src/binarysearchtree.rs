
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


    pub fn inorder(&self) {
        Bst::_inorder(&self.root);
    }

    pub fn _inorder(curr: &Option<Rc<RefCell<TNode>>>) {
        if let Some(ref link) = curr {
            let left  = link.borrow().left.as_ref().map(|x| x.clone());
            let right = link.borrow().right.as_ref().map(|x| x.clone());

            Bst::_inorder(&left);
            print!(" {} ", &link.borrow().val);
            Bst::_inorder(&right);
        }
    }

    pub fn insert(&mut self, val: i32) {
        //let new_node = Bst::new_link(val);

        match self.root.take() {
            None => {
                print!("empty tree - gets first val of {}!\n", &val);
                self.root = Some(Bst::new_link(val));
            },
            Some(root) => {
                print!("non-empty list, inserting val of {}...\n", &val);
                let mut curr_wrapped = Some(root.clone());

                //restore root
                self.root = Some(root);

                while let Some(curr) = curr_wrapped {
                    print!("comparing {} to {}\n", &val, &curr.borrow().val);
                    let ans = val.cmp(&curr.borrow().val);
                    match ans {
                        Ordering::Less => {
                            print!("\tit is LESS!\n");
                            if let Some(n) = curr.borrow().left.as_ref().map(|x| x.clone()) {
                                print!("\tleft tree is NOT NULL. recursing...\n");
                                curr_wrapped = Some(n);
                                continue;
                            }

                            print!("\tleft subtree is null.  inserting {} here!\n", &val);
                            curr.borrow_mut().left = Some(Bst::new_link(val));
                            break;
                        },
                        Ordering::Equal | Ordering::Greater => {
                            print!("\tit is EQUAL or GREATER!\n");
                            if let Some(n)= curr.borrow().right.as_ref().map(|x| x.clone()) {
                                print!("\tright tree is NOT NULL. recursing...\n");
                                curr_wrapped = Some(n);
                                continue;
                            }

                            print!("\tright subtree is null.  inserting {} here!\n", &val);
                            curr.borrow_mut().right = Some(Bst::new_link(val));
                            break;
                        },
                    } //match val.cmp(..)

                } //while

            } //Some(root)
        } //match
    } //insert

} //impl




pub struct InorderIter(Option<Link>);
impl InorderIter {
    
}

impl Iterator for InorderIter {
    type Item = i32;
    
    fn next(&mut self) -> Option<i32> {




        None
    }

}








pub fn do_bst() {
    
    let mut bst = Bst::new();

    for x in vec![42, 12, 30, 44, 1000, -5] {
        bst.insert(x);
    }
    
    print!("\n\ninorder:\n\t");
    bst.inorder();
    print!("\n\n");

}