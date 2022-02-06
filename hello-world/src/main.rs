mod find_gcd;
mod hello;
mod hello_parallel;
mod hello_println;

use std::env;
use std::str::FromStr;

fn main() {
    hello::hello();
    hello_println::hello_println();
    hello_parallel::hello_parallel();

    let mut args = env::args();

    if args.len() < 3 {
        let n: u64 = 20;
        let m: u64 = 12;
        println!("gcd({},{}) = {}", n, m, find_gcd::gcd(n, m));
        // println!("gcd({},{}) = {}", n, 0, find_gcd::gcd(n, 0));  // causes panic
    } else if args.len() == 3 {
        let n: u64 = u64::from_str(&args.nth(1).unwrap()).expect("error parsing argument");
        let m: u64 = u64::from_str(&args.nth(2).unwrap()).expect("error parsing argument");
        println!("gcd({},{}) = {}", n, m, find_gcd::gcd(n, m));
    } else {
        let mut n: u64 = u64::from_str(&args.nth(1).unwrap()).expect("error parsing argument");
        for arg in args.skip(2) {
            let m: u64 = u64::from_str(&arg).expect("error parsing argument");
            let t = find_gcd::gcd(n, m);
            println!("gcd({},{}) = {}", n, m, t);
            n = t;
        }
    }
}

#[test]
fn test_gcd() {
    assert_eq!(find_gcd::gcd(14, 15), 1);

    assert_eq!(
        find_gcd::gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19),
        3 * 11
    );
}
