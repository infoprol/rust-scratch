
pub mod priority_heap;

use priority_heap::PriorityQueue;

pub mod scratch;
use scratch::MyBox;

pub mod bst;

pub mod ll;
use ll::LNode;
use ll::ll_scratch;

pub mod quick;

pub mod linkedlist;
use linkedlist::LList;
use linkedlist::do_ll;





fn q() {

    for j in 0 .. 17 {
        let (x,y) = PriorityQueue::<String>::children(j as usize);
        print!("children({}) - {}, {}\n", j, x as i32, y as i32);
        
        let p = PriorityQueue::<String>::parent(j as usize);
        print!("parent({}) - {}\n\n", j, p as i32);
    }


    let mut pq = PriorityQueue::<>::new();
    print!("got new pq!");

    pq.insert("3", 3);
    pq.insert("1", 1);
    pq.insert("4", 4);
    pq.insert("2", 2);
    pq.insert("37", 37);
    pq.insert("12", 12);
    pq.insert("13", 13);


    print!("popping...\n");
    while let Some(x) = pq.pop() {
        print!("POP {}\n", x.elem);
    }

}

fn main() {

    do_ll();
    //ll_scratch();
    //box_stuff();

    //quick::sort_quickly();
}


fn hello(name: &str) {
    println!("hello, {}!", name);
}

fn box_stuff() {
    let x = MyBox::new(String::from("zaphod"));
    hello(&x);

    let y = &(*x)[..];

    assert_eq!(&(*x), y);
}