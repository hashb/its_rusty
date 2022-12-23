pub fn references() {
    println!("..::references::..");

    let mut x: i32 = 11;
    let mut y: i32 = 99;

    println!("x: {}, y: {}", x, y);

    // borrowing scope
    {
        let ref_x: &mut i32 = &mut x;
        let ref_y: &mut i32 = &mut y;
        let temp: i32;

        println!("x: {}, y: {}", *ref_x, *ref_y);

        // &mut means borrowing
        // following code doesnt work because
        // x and y are borrowed by ref_x and ref_y
        // *ref_x = y;
        // *ref_y = x;
        // x = 13;

        temp = *ref_x;
        *ref_x = *ref_y;
        *ref_y = temp;

        println!("ref_x: {}, ref_y: {}", *ref_x, *ref_y);
        println!("ref_x: {:b}, ref_y: {:b}", *ref_x, *ref_y);
        println!(
            "ref_x: {}, ref_y: {}",
            ref_x.count_ones(),
            ref_y.count_ones()
        );
        println!("ref_x: {:b}, ref_y: {:b}", ref_x, ref_y);
    }
    // once borrowers are out of scope,
    // ownership returns to original variable
    x *= 10;
    y *= 10;

    println!("x: {}, y: {}", x, y);
    println!("x: {}, y: {}", x.count_ones(), y.count_ones());
    println!("x: {:b}, y: {:b}", x, y);
    println!();
}

// pub fn dangling_references() {
//     let ref_x: &i32;
//     {
//         let x: i32 = 10;
//         // doesnt work because x doesnt live long enough
//         // for the reference to be valid.
//         // we are creating a dangling reference
//         ref_x = &x;
//     }
//     println!("ref_x: {ref_x}");
// }
