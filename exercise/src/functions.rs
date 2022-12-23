use rand::Rng;

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    // if a semicolon is not added on the last expression
    // of a function, that value is used as ther return value
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) {
    // pattern matching
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true, true) => println!("fizzbuzz"),
        (true, false) => println!("fizz"),
        (false, true) => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}

fn fizbuzzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
    println!();
}

struct Rect {
    w: u32,
    h: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn increment_width(&mut self, delta: u32) {
        self.w += delta;
    }
}

fn methods() {
    let mut rect = Rect { w: 10, h: 5 };
    println!("rect w: {}, h: {} area: {}", rect.w, rect.h, rect.area());
    rect.increment_width(10);
    println!("rect w: {}, h: {} area: {}", rect.w, rect.h, rect.area());
    println!();
}

fn pick_one<T>(rand_num: u32, a: T, b: T) -> T {
    if rand_num % 2 == 0 {
        a
    } else {
        b
    }
}

fn overloading() {
    let mut rng = rand::thread_rng();
    let rand_num: u32 = rng.gen();
    println!("coin toss: {}", pick_one(rand_num, "heads", "tails"));
    let rand_num: u32 = rng.gen();
    println!("cash prize: {}", pick_one(rand_num, 500, 1000));
    // println!("cash prize: {}", pick_one(-32, 500, 1000));
    println!();
}

pub fn functions() {
    println!("..::functions::..");
    fizbuzzz_to(16);
    methods();
    overloading();
}

// Functions dont support overloading
// Functions dont support default values
// templates are supported
