use std::fmt::Display;
use std::fmt::Debug;
use array_init;


pub struct Node<T> {
    pub elem: T,
    pub priority: i32,
}
pub type ArrEntry<T> = Option<Box<Node<T>>>;
pub struct PriorityQueue<T> {
    size: usize,
    heap: [ArrEntry<T>; 64],
}



impl<T: Display> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue {
            size: 0,
            heap: array_init::array_init(|_| None)
        }
    }

    pub fn parent(k: usize) -> usize {
        if k < 1 { return 0 as usize; }
        (k - 1) / 2 as usize
    }

    pub fn children(k: usize) -> (usize, usize) { (2 * k + 1, 2 * k + 2) }

    pub fn insert(&mut self, elem: T, priority: i32) {
        let arr_ent = Some(Box::new(Node {
            elem: elem,
            priority: priority,
        }));
        self.heap[self.size] = arr_ent;
        self.size = self.size + 1;

        self.bubble_up();
    }
    pub fn bubble_up(&mut self) {
        let mut k = self.size - 1;
        
        // need to bubble up
        // <=>
        // curr is higher priority than parent
        // <=>
        // curr.priority > pant.priority
        // <=>
        // c.p - p.p > 0

        print!("bubbling up!\n");
        while k > 0 {
            
            let curr = &self.heap[k];
            let mut p = PriorityQueue::<T>::parent(k);
            let pant = &self.heap[p];
            
            let diff = match (*curr).as_ref().zip((*pant).as_ref())
                .map(|(c,p)| c.priority  -  p.priority) {
                    Some(priority_diff) => priority_diff,
                    None => 0
                };
            
            if diff <= 0 { break; }
            //else swap
            let curr_elem = (*curr).as_ref().map(|x| &x.elem).unwrap();
            let pant_elem = (*pant).as_ref().map(|x| &x.elem).unwrap();

            print!("swapping {}:{} and {}:{}\n", k, curr_elem, p, pant_elem);
            self.heap.swap(k, p);
            
            //move indexes one level up tree
            k = p;
            p = PriorityQueue::<T>::parent(p);
        }
        ;
    }

    pub fn compare(lhs: &ArrEntry<T>, rhs: &ArrEntry<T>) -> i32 {
        match (*lhs).as_ref().zip((*rhs).as_ref())
            .map(|(l,r)| l.priority - r.priority) {
                Some(diff) => diff,
                None => 0
            }    
    }

    pub fn max_child(&self, k: usize) -> Option<usize> {
        let (l, r) = PriorityQueue::<T>::children(k);
        print!("max_child({}) -> ({},{}). size={}\n", k as i32, l as i32, r as i32, self.size);
        
        if self.size <= l {
            print!("self.size <= l.  returning None!\n");
            return None
        }
        if self.size <= r {
            print!("l < self.size <= r. returning Some({}).\n", l as i32);
            return Some(l)
        }


        let diff = PriorityQueue::<T>::compare(
            &self.heap[l],
            &self.heap[r]);
        print!("max_child.diff({},{}) = {}.\n", l as i32, r as i32, diff);
        if diff > 0 { return Some(l) }
        Some(r)
    }

    pub fn push_down(&mut self) {
        let mut k: usize = 0;
        while k < self.size {
            let diff = match self.max_child(k) {
                None => {
                    print!("max_child({}) was None, so push_down.diff = 0", k as i32);
                    0
                },
                Some(child) => {
                    print!("max_child({})={}, comparing...", k as i32, child as i32);
                    let diff = PriorityQueue::<T>::compare(&self.heap[child], &self.heap[k]);
                    print!("push_down.compare({},{})={}, so using that.", child as i32, k as i32, diff);
                    diff
                }, 
            };

            print!("push_down.  diff={}, k={}.\n", diff, k as i32);
            //k is higher priority than either of its children (we're done)
            if diff <= 0 { break; }

            let child = self.max_child(k).unwrap();
            self.heap.swap(child, k);
            print!("swapped {} and {}.  now setting k={}", child as i32, k as i32, child);
            k = child;

            print!("diff > 0, so we keep going.  k now {}.  self.size={}.\n", k as i32, self.size);
        }
        print!("push_down doneyet.  k={}\n", k as i32);
    }


    pub fn pop(&mut self) -> ArrEntry<T> {
        if self.size < 1 { return None; }

        let ans = self.heap[0].take();
        self.heap[0] = self.heap[self.size - 1].take();
        self.size = self.size - 1;

        self.push_down();

        ans
    }





        //let bub_pri = self.heap[k].priority;
        //let p = parent(k);
    
        /*
        while k > 0 {
            let p = PriorityQueue::<T>::parent(k);
            let curr_node = self.heap[k].map(|n| n);
            let par_node = self.heap[p].map(|n| n);

            let par_prio = par_node.map(|n| n.priority);
            let cur_prio = curr_naaaaaode.map(|n| n.priority);

            if par_prio >= cur_prio { break; }

            self.heap[k] = par_node;
            self.heap[p] = curr_node;
            k = p;
        }
        */

    


}



#[cfg(test)]
mod test {
    use super::PriorityQueue;
    
    #[test]
    fn basics() {
        assert_eq!(42, 42);

        for j in 0 .. 17 {
            let (x,y) = PriorityQueue::<String>::children(j as usize);
            
            print!("children({}) - {}, {}", j, x as i32, y as i32);
        }
/*
        let mut pq = PriorityQueue::<String>::new();
        let mut k: i32 = 0;
        while k < 10 {
            pq.insert(k.to_string(), k);
            print!("inserted {}", k.to_string());
            k = k + 1;
        }

        while let Some(bn) = pq.pop() {
            let elem = bn.elem;

            print!("{}", elem);
        }
*/
    }
}
