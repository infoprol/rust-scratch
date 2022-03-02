
//in-place quicksort
fn quicksort(ll: &mut [i32]) {
    println!("ll.len() ~> {}", ll.len());
    print_arr("-", &ll[..]);

    if ll.len() == 2 {
        if ll[0] > ll[1] {
            ll.swap(0,1);
        }
    }
    if ll.len() < 3 { return; }

    let pivot = ll[0];
    let mut i = 0;
    for j in 1..ll.len() {
        if ll[j] < pivot {
            ll.swap(i,j);
            i += 1;
        }
    }
    
    //then we just need to sort 1..
    if i == 0 {
        quicksort(&mut ll[1..]);
        return;
    }

    quicksort(&mut ll[0..i]);
    quicksort(&mut ll[i..]);
}


fn print_arr(arr_name: &str, arr: &[i32]) {
    println!("arr {}:", arr_name);
    for j in 0..arr.len() {
        println!("{}", arr[j]);
    }
    println!("~~\n");
}

pub fn sort_quickly() {
    let mut ll = [42, -666, 69, 420, 
                1000, -54, -3, 8, 1111, 111, 11, 1];

    quicksort(&mut ll[..]);

    let mm = &ll[0..4];
    print_arr("mm", &mm);
    
    let nn = &ll[4..];
    print_arr("nn", &nn);

    for j in 0..ll.len() {
        println!("{}", ll[j]);
    }

    print_arr("ans", &ll[..]);

    println!("ok");
}
