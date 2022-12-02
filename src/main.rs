#![allow(unused_parens)]

pub mod day01;
pub mod day02;

fn main() {
    let (a, b) = day01::soln("data/day01.txt");
    println!("Day 01");
    println!("\tPart A: {a}");
    println!("\tPart B: {b}");   

    let (a, b) = day02::soln("data/day02.txt");
    println!("Day 02");
    println!("\tPart A: {a}");
    println!("\tPart B: {b}");

}
