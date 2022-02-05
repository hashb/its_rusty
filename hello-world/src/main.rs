mod hello;
mod hello_parallel;
mod hello_println;

fn main() {
    hello::hello();
    hello_println::hello_println();
    hello_parallel::hello_parallel();
}
