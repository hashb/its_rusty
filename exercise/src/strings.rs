pub fn strings() {
    println!("..::strings::..");

    let s1: &str = "hello";
    println!("s1: {s1}");

    let mut s2: String = String::from("hello string");
    println!("s2: {s2}");

    // &str an immutable reference to a string slice.
    // String a mutable string buffer.
    s2.push_str(s1);
    println!("s2: {s2}");
    println!();
}
