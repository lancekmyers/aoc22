use std::io::Read;

use crate::aoc;

fn all_distinct(xs : &[u8]) -> Option<usize> {
    let n = xs.len();
    for i in 0..n {
        for j in (i+1)..n {
            if (xs[i] == xs[j]) {
                return Some(i)
            }
        }
    }
    None
}


pub struct Day06 {}
pub const DAY_06 : Day06 = Day06{};
impl aoc::Soln for Day06 {
    type I = String;

    type A = usize;

    type B = usize;

    const NAME : &'static str = "Day six";

    fn soln_a(&self, input : &Self::I) -> Self::A {
        let mut i = 4;
        for x in input.as_bytes().windows(4) {
            if let [a, b, c, d] = x {
                let any_same = (a == b || a == c || a == d) ||
                    (b == c || b == d) || (c == d);
                if !any_same { 
                    break;
                } else {
                    i += 1
                }
            }
        }
        i
    }

    fn soln_b(&self, input : &Self::I) -> Self::B {
        let xs = input.as_bytes();
        let mut i = 0;
    
        while let Some(jump) = all_distinct(&xs[i..i+14]) {
            i += jump + 1 
        }
        
        i + 14
    }

    fn parse(&self, file : &mut std::fs::File) -> Self::I {
        let mut buf = String::new();
        _ = file.read_to_string(&mut buf);
        buf
    }
}

 

#[cfg(test)]
mod tests{
    use crate::day06::*;
    use aoc::Soln;
    #[test]
    fn given_a() {
        assert_eq!(DAY_06.soln_a(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 7);
        assert_eq!(DAY_06.soln_a(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(DAY_06.soln_a(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
        assert_eq!(DAY_06.soln_a(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
        assert_eq!(DAY_06.soln_a(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);
    }
    
    #[test]
    fn given_b() {
        assert_eq!(DAY_06.soln_b(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 19);
        assert_eq!(DAY_06.soln_b(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 23);
        assert_eq!(DAY_06.soln_b(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 23);
        assert_eq!(DAY_06.soln_b(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 29);
        assert_eq!(DAY_06.soln_b(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 26);
    }

    #[test]
    fn all_distinct_tests() {
        assert_eq!(None, all_distinct("abcdefg".as_bytes()));
        assert_eq!(None, all_distinct("".as_bytes()));
        assert_eq!(Some(0), all_distinct("aba".as_bytes()))
    }
}
