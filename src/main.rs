#![allow(unused_parens)]

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

fn main() {
    let (a, b) = day01::soln("data/day01.txt");
    println!("Day 01");
    println!("\tPart A: {a}");
    println!("\tPart B: {b}");   

    let (a, b) = day02::soln("data/day02.txt");
    println!("Day 02");
    println!("\tPart A: {a}");
    println!("\tPart B: {b}");

    let (a, b) = day03::soln("data/day03.txt");
    println!("Day 03");
    println!("\tPart A: {a}");
    println!("\tPart B: {b}");

    let (a, b) = day04::soln("data/day04.txt");
    println!("Day 04");
    println!("\tPart A: {a}");
    println!("\tPart B: {b}");

    if let Some((a, b)) = day05::soln("data/day05.txt") {
        println!("Day 05");
        println!("\tPart A: {a}");
        println!("\tPart B: {b}");
        
    }
    
    let (a, b) = day06::soln("data/day06.txt");
    println!("Day 06");
    println!("\tPart A: {a}");
    println!("\tPart B: {b}");
    
}
