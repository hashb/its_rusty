#![allow(dead_code)]

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn implicit_conversions() {
    let x: f32 = 99.432098;
    let y: u8 = 42;

    // rust doesnt allow implicit conversions
    // println!("{x} x {y} = {}", multiply(x, y));
    // cannot convert from float to int?
    println!("{x} x {y} = {}", multiply(x as i32, y.into()));
    println!("{x} x {y} = {}", multiply(x.round() as i32, i32::from(y)));
    println!();
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut ret = matrix;
    for i in 0..ret.len() {
        for j in 0..ret[i].len() {
            ret[i][j] = matrix[j][i];
        }
    }
    ret
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        for c in row {
            print!("{c}, ");
        }
        println!();
    }
    println!();
}

fn pretty_print_slice(matrix: &[&[i32]]) {
    for row in matrix.iter() {
        for c in row.iter() {
            print!("{c}, ");
        }
        println!();
    }
    println!();
}

fn array_and_for_loops() {
    let matrix = [
        [101, 102, 103], // <-- the comment make rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);
    // we need to make slices of slices
    // pretty_print_slice(&matrix.as_slice());

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
    println!();
}

pub fn day01am_exercises() {
    println!("..::exercies - day 01 morning::..");

    implicit_conversions();
    array_and_for_loops();
}
