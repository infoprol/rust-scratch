

use std::rc::Rc;
use std::cell::RefCell;


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
        else {
            self.head = Some(Box::new(LNode {
                val: val,
                next: None
            }));
        }
    
    }
}



pub fn ll_scratch() {
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













pub struct LIter<'a> {
    next: Option<&'a LNode>,
}

/*
impl<'a, i32> Iterator for LIter<'a> {
    type Item = &'i32;
    fn next(&'a mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            let val = n.val;
            self.next = n.next;
            &val
        })
    }
}
*/



/*
impl LList {
    fn new () -> Self { LList::Nil }

    fn insert(&mut self, new_val: i32) {
        match self {
            LList::LNode {
                ref val,
                ref mut next,
            } if new_val >= *val => {
                print!("val={}\tnew_valval={}\n", val, new_val);
                if new_val >= *val {
                    print!("new val {} >= val of {}; recursing...\n", new_val, val);
                    next.insert(new_val);
                }
            },
            xx => { print!("argg"); },
        }
    }
}

*/