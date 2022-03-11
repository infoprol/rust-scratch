

use std::rc::Rc;
use std::cell::RefCell;



type NodeLink = Rc<RefCell<LLLNode>>;
pub struct LLLNode {
    elem: i32,
    next: Option<NodeLink>,
}
pub struct LLList {
    head: Option<NodeLink>,
}

impl LLList {
    pub fn new() -> Self { LLList { head: None, }}

    pub fn insert(&mut self, val: i32) {
        let mut new_node = Rc::new(RefCell::new(LLLNode {
            elem: val,
            next: None,
        }));


        match &self.head {
            None => {
                self.head = Some(Rc::clone(&new_node));
            },
            Some(node_link) => {
                let mut curr = Some(Rc::clone(&node_link));
                let mut prev = self.head.as_ref().map(|x| Rc::clone(&x));
                loop {
                    match curr {
                        Some(nl) => {
                            let c = nl.borrow();
                    
                            if val < c.elem {
                                let new_node = LLLNode {
                                    elem: val,
                                    next: Some(Rc::clone(&nl)),
                                };
                                prev.map(|x| { x.borrow_mut().next = Some(Rc::new(RefCell::new(new_node))); });
                                //(*nl.borrow_mut()).next = Some(Rc::new(RefCell::new(new_node)));
                            }
                            
                            //prev = curr;
                            prev = Some(Rc::clone(&nl));
                            curr = c.next.as_ref().map(|x| Rc::clone(&x));
                        },
                        None => { break; }


                    }                    
                }
            }
        }
    }

    pub fn traverse(&self) {
        let mut curr = self.head.as_ref().map(|x| Rc::clone(&x));
        loop {
            match curr {
                None => {
                    print!("//\n");
                    break;
                },
                Some(node_link) => {
                    print!("{} -> ", node_link.borrow().elem);
                    curr = node_link.borrow().next.as_ref().map(|x| Rc::clone(&x));
                }
            }
        }
    }

}




pub fn ll_scratch() {
    let mut ll = LLList::new();
    ll.insert(11);
    ll.insert(1);
    ll.insert(100);
    ll.insert(3);
    ll.insert(5);
    ll.insert(2);
    ll.insert(7);

    print!("traversing list...\n");
    ll.traverse();
}











pub fn ll_scratchaaaa() {
    let mut twenty = LLLNode { elem: 20, next: None };
    let mut ten = LLLNode { elem: 10, next: Some(Rc::new(RefCell::new(
        twenty
    ))) };
    let mut one = LLLNode { elem: 1, next: Some(Rc::new(RefCell::new(
        ten
    ))) };


    
    let link_to_ten = (&one).next.as_ref().map(|t| Rc::clone(&t));
    one.next = Some(Rc::new(RefCell::new(LLLNode {
        elem: 5,
        next: link_to_ten,
    })));

    let mut head = Some(Rc::new(RefCell::new(one)));
    
    let mut curr = head.map(|x| Rc::clone(&x));
    //while let Some(curr) = curr {
    loop {
        match curr {
            Some(n) => {
                let c = n.borrow();
                print!("{}\n", c.elem);
                curr = c.next.as_ref().map(|x| Rc::clone(&x));
            },
            None => { break; }
        }
    }

}


type ListLink = Option<Rc<ListNode>>;

#[derive(Debug)]
struct ListNode {
    elem: i32,
    next: ListLink,
}


pub fn ll_scratchxx() {

    let mut one = ListNode { elem: 1, next: None };
    let mut two = ListNode { elem: 2, next: None };
    let mut three = ListNode { elem: 3, next: None };

    let link_to_three = Some(Rc::new(three));
    two.next = link_to_three;

    let link_to_two = Some(Rc::new(two));
    one.next = link_to_two;

    let mut head = Some(Rc::new(one));

    print!("built our list...\n");

    let mut curr = head.as_ref().map(|x| x.clone());
    while let Some(curr) = curr.as_ref().map(|x| x.next.clone()) {
        if let Some(val) = curr.as_ref().map(|x| x.elem) {
            print!("curr.elem = {}\n", val);
        }
    }



    head.as_ref().zip(curr.as_ref()).map(|(h,c)| {
        let hval = h.as_ref().elem;
        let cval = c.as_ref().elem;

        print!("head at {}\ncurr at {}\n", hval, cval);

        let hnext = h.as_ref().next.clone();
        if let Some(hnextval) = hnext.as_ref().map(|x| x.elem) {
            print!("hnext.elem = {}\n", hnextval);
        }
    });

    


}



//////
/// this seems to work
pub type LLink = Option<Box<LNode>>;
pub struct LNode {
    val: i32,
    next: LLink,
}
pub struct LList {
    head: LLink,
}

impl LNode {
    fn insert(&mut self, val: i32) {

        match self.next {
            None => self.next = Some(Box::new(LNode { val: val, next: None })),
            Some(ref mut node) => {
                if val < node.val {
                    let new_node = LNode { val: val, next: self.next.take() };
                    self.next = Some(Box::new(new_node));
                }
                else {
                    node.insert(val);
                }
            }
        }
    }

    fn traverse(&self) {
        print!("{} -> ", self.val);
        if let Some(ref node) = self.next { node.traverse(); }
        else { print!("//\n"); }
    }
}

impl LList {
    fn new() -> Self { LList { head: None } }
    fn insert(&mut self, val: i32) {
        if let Some(mut node) = self.head.as_ref() {
            if val < node.val {
                let new_node = LNode { val: val, next: None };
                self.head = Some(Box::new(new_node));
                
            //node.insert(val);
            }
        }
        else {
            self.head = Some(Box::new(LNode {
                val: val,
                next: None
            }));
        }
    
    }
}



pub fn ll_scratchyyy() {
    let mut head = LNode { val: 17, next: None };
    head.insert(11);
    head.insert(1);
    head.insert(100);
    head.insert(3);
    head.insert(5);
    head.insert(2);
    head.insert(7);

    print!("traversing list...\n");
    head.traverse();
}






