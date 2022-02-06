mod find_gcd;
mod hello;
mod hello_parallel;
mod hello_println;

fn main() {
    hello::hello();
    hello_println::hello_println();
    hello_parallel::hello_parallel();
    let n: u64 = 20;
    let m: u64 = 12;
    println!("gcd({},{}) = {}", n, m, find_gcd::gcd(n, m));
    // println!("gcd({},{}) = {}", n, 0, find_gcd::gcd(n, 0));  // causes panic
}

#[test]
fn test_gcd() {
    assert_eq!(find_gcd::gcd(14, 15), 1);

    assert_eq!(
        find_gcd::gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19),
        3 * 11
    );
}
