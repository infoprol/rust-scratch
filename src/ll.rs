

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

