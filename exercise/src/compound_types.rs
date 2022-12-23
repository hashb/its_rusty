pub fn compound_types() {
    println!("..::compound_types::..");

    // let mut a: [i32; 10] = [11; 11];
    let mut a: [i32; 10] = [11; 10];
    a[1] = 1;
    println!("a: {:?}", a);
    println!();

    let t: (i32, &str, &[u8]) = (42, r#"loremiafds"fadsfads''"#, b"adsfa");
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
    println!("t.2: {:?}", t.2);
    println!();
}
