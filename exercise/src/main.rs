mod compound_types;
mod day01am_exercises;
mod functions;
mod references;
mod slices;
mod small_example;
mod strings;

fn main() {
    println!();
    // intro
    println!("Hello, 🌍!");
    println!();

    // 4.1 small example
    small_example::small_example();

    // 6.2 compound types
    compound_types::compound_types();

    // 6.3 references
    references::references();

    // 6.4 slices
    slices::slices();

    // 6.4.1 strings
    strings::strings();

    // 6.5 functions()
    functions::functions();

    // Day 01 Morning Exercises
    day01am_exercises::day01am_exercises();
}
