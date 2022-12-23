pub fn slices() {
    println!("..::slices::..");

    let mut a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("a: {a:?}");

    {
        // take a slice
        let s: &[i32] = &a[2..4];
        println!("s: {s:?}");
        // slices are mutable
        // slices are not borrows?
        // A slice is a kind of reference, so it does not have ownership.
        // https://doc.rust-lang.org/book/ch04-03-slices.html
        a[0] = 99;
        a[3] = 101;
        println!("a: {a:?}");
    }
    a[0] = 999;
    a[3] = 1011;
    println!("a: {a:?}");
    println!();
}
