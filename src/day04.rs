use std::{fs::File, io::{BufReader, BufRead}};

/*
use nom;
use nom::character::complete::{char, digit0};
use nom::{IResult, sequence::delimited, character::complete};
*/
#[derive(Debug, PartialEq, Clone, Copy)]
struct Interval {
    begin : i32, 
    end : i32
}

impl Interval {
    fn contains(self : &Interval, other : & Interval) -> bool {
        other.begin >= self.begin && other.end <= self.end
    }
}

fn overlaps(x : Interval, y : Interval) -> bool {
    let case1 = y.begin <= x.end && x.begin <= y.end;
    let case2 = x.begin <= y.end && y.begin <= x.end;
    case1 || case2
}

fn parse_interval(input : &str) -> Option<Interval> {
    let x : Vec<i32> = input.split('-').flat_map(|x| x.parse().ok()).collect();
    // nom::character::is_digit(chr)
    Some(Interval { begin : *x.first()?, end : *x.get(1)? })
}

fn parse_line(input : &str) -> Option<(Interval, Interval)> {
    let x : Vec<Interval> = input.split(',').flat_map(parse_interval).collect();
    
    Some(( *x.get(0)?, *x.get(1)? ))
}

pub fn soln(path : &str) -> (i32, i32) {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines().flatten(); //lines(file);

    let a = lines.flat_map(|x| parse_line(&x)).filter(
        |(x, y) | x.contains(y) || y.contains(x)
    ).count();

    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines().flatten(); 
    let b = lines.flat_map(|x| parse_line(&x)).filter(
        |(x, y) | overlaps(*x,*y)
    ).count();

    (a.try_into().unwrap(), b.try_into().unwrap())
    // s(a, b)
}



#[cfg(test)]
mod tests{
    use crate::day04::*;
    #[test]
    fn parsing() {
        assert_eq!(
            Some(Interval {begin:12, end:30}), 
            parse_interval("12-30")
        );

        assert_eq!(
            Some((Interval {begin:12, end:30}, Interval {begin:5, end:20})), 
            parse_line("12-30,5-20")
        )
    }

    #[test]
    fn overlaping() {
        assert_eq!(
            true, 
            overlaps(Interval{begin: 5, end: 7}, Interval{begin:7, end:9})
        );
        assert_eq!(
            true, 
            overlaps(Interval{begin: 2, end: 8}, Interval{begin:3, end:7})
        );
        assert_eq!(
            true, 
            overlaps(Interval{begin: 6, end: 6}, Interval{begin:4, end:6})
        );
        assert_eq!(
            false, 
            overlaps(Interval{begin: 2, end: 3}, Interval{begin:4, end:6})
        );
    }

    #[test]
    fn given() {
        assert_eq!((2,4), soln("data/test04.txt"))
    }
}



