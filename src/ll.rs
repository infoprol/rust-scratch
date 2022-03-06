

use std::rc::Rc;
use std::cell::RefCell;


type ListLink = Option<Rc<ListNode>>;


#[derive(Debug)]
struct ListNode {
    elem: i32,
    next: ListLink,
}


pub fn ll_scratch() {

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
            print!("curr.elem = {}", val);
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










pub enum LList {
    LNode {
        val:    i32,
        next:   Box<LList>, },
    Nil, }


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

