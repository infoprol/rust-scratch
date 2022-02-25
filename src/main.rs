
pub mod priority_heap;

use priority_heap::PriorityQueue;


fn main () {

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