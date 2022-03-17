
use std::rc::Rc;
use std::cell::RefCell;


type LLink = Rc<RefCell<LNode>>;
pub struct LNode {
    elem: i32,
    next: Option<LLink>,
}
pub struct LList {
    head: Option<LLink>,
}

impl LNode {
    pub fn new(elem: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(LNode {
            elem: elem,
            next: None,
        }))
    }
}

impl LList {
    pub fn new() -> Self { LList { head: None, } }


    pub fn remove(&mut self, elem: i32) {
        match self.head.take() {
            None => {
                print!("trying to delete {} from an empty list!\n", elem);
                //need to put head back??
                self.head = None;
            },
            Some(n) if elem == n.borrow().elem => {
                print!("removing {} from the head of the list!\n", elem);
                self.head = n.borrow().next.as_ref().map(|x| Rc::clone(&x));
            },
            Some(n) => {
                self.head = Some(Rc::clone(&n));
                let mut curr = Rc::clone(&n);
                loop {

                    let mut next = curr.borrow().next.as_ref().map(|x| Rc::clone(&x));
                    match next.take() {
                        None => {
                            print!("got to the end of the list without finding {} to remove!\n", elem);
                            //replace next?
                            curr.borrow_mut().next = None;
                            break;
                        },
                        Some(m) if elem == m.borrow().elem => {
                            print!("ha!  found {} and am removing it from middle of list!\n", elem);
                            curr.borrow_mut().next = m.borrow().next.as_ref().map(|x| Rc::clone(&x));
                            //keep iterating (might be duplicates)
                        },
                        Some(m) if elem < m.borrow().elem => {
                            print!("we are past where {} would be, so terminating!\n", elem);
                            //replace next?
                            curr.borrow_mut().next = Some(Rc::clone(&m));
                            break;
                        },
                        //else keep chugging along...
                        Some(m) => {
                            print!("still looking for {}...\n", elem);
                            //restore next?
                            curr.borrow_mut().next = Some(Rc::clone(&m));
                            curr = Rc::clone(&m);
                        }
                    }

                }
            }
        }
    }



    pub fn insert(&mut self, elem: i32) {
        let new_node = LNode::new(elem);
        match self.head.take() {
            None => {
                self.head = Some(new_node);
                print!("in the empty list case.\n");
            },
            Some(n) if elem < n.borrow().elem => {
                print!("adding {} at the beginning of the list.\n", elem);
                new_node.borrow_mut().next = Some(Rc::clone(&n));  //self.head.as_ref().map(|x| Rc::clone(x));
                self.head = Some(new_node);
            },
            Some(head) => {
                print!("adding {} in the middle of the list...\n", elem);
                let mut link = Some(Rc::clone(&head));
                self.head = Some(head);

                loop {
                    
                    let curr = link.unwrap();
                    {
                        print!("inserting {}; curr pointed to {}\n", elem, curr.borrow().elem);
                    }
                    {
                        let mut n = curr.borrow_mut();
                        if let Some(m) = &n.next {
                            if elem < m.borrow().elem {
                                print!("inserting {} right before {}.\n", elem, m.borrow().elem);
                                new_node.borrow_mut().next = Some(Rc::clone(&m));
                                n.next = Some(new_node);
                                break;
                            }
                        }
                        if let None = &n.next {
                            print!("got to the end of the list.  inserting {} at the end.\n", elem);
                            n.next = Some(new_node);
                            break;
                        }
                    }
                    link = curr.borrow().next.as_ref().map(|x| Rc::clone(&x));

                }

            }
        }
    }

    pub fn traverse(&self) {
        let mut curr = self.head.as_ref().map(|x| Rc::clone(x));
        loop {
            match curr {
                Some(n) => {
                    print!("{} -> ", n.borrow().elem);
                    curr = n.borrow().next.as_ref().map(|x| Rc::clone(&x));
                },
                None => {
                    print!("//\n");
                    break;
                }
            }
        }
    }

    pub fn iter(self) -> Iter {
        Iter(self.head.as_ref().map(|x| Rc::clone(&x)))
    }
}



pub struct Iter(Option<LLink>);
impl Iterator for Iter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        match &self.0.take() {
            None => { return None; },
            Some(lnk) => {
                let ans = lnk.borrow().elem;
                self.0 = lnk.borrow().next.as_ref().map(
                    |x| Rc::clone(x)
                );
                return Some(ans);
            }
        }
    }
}




/*
impl Drop for LList {
    fn drop(&mut self) {
    }
}
*/



pub fn do_ll() {
    let mut ll = LList::new();
    ll.insert(42);
    ll.insert(3);
    ll.insert(1);
    ll.insert(12);
    ll.insert(100);
    ll.insert(7);

    ll.traverse();

    let mut jj = LList::new();

    print!("first itering:\n");
    for j in ll.iter() {
        print!("ll:{}\n", j);
        jj.insert(j);
    }

    let mut mm = LList::new();
    print!("second itering:\n");
    for j in jj.iter() {
        print!("j:{}\n", j);
        mm.insert(j);
    }

    mm.remove(12);
    mm.remove(100);
    print!("after removing 12 and 100:\n");
    mm.traverse();
    jj = LList::new();




}