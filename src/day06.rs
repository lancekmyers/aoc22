
fn all_distinct(xs : &[u8]) -> Option<usize> {
    let n = xs.len();
    for i in 0..n {
        for j in (i+1)..n {
            if (xs[i] == xs[j]) {
                return Some(i)
            }
        }
    }
    return None
}

fn soln_a(str : &str) -> usize {
    let mut i = 4;
    for x in str.as_bytes().windows(4) {
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
    return i;
}

fn soln_b(str : &str) -> usize {
    let xs = str.as_bytes();
    let mut i = 0;

    while let Some(jump) = all_distinct(&xs[i..i+14]) {
        i += jump + 1 
    }
    
    return i + 14;
}


pub fn soln(path : &str) -> (usize, usize) {
    let input = std::fs::read_to_string(path).unwrap();
    let a = soln_a(&input);
    let b = soln_b(&input);
    (a, b)
} 

#[cfg(test)]
mod tests{
    use crate::day06::*;
    #[test]
    fn given_a() {
        assert_eq!(soln_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(soln_a("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(soln_a("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(soln_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(soln_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    
    #[test]
    fn given_b() {
        assert_eq!(soln_b("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(soln_b("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(soln_b("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(soln_b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(soln_b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn all_distinct_tests() {
        assert_eq!(None, all_distinct("abcdefg".as_bytes()));
        assert_eq!(None, all_distinct("".as_bytes()));
        assert_eq!(Some(0), all_distinct("aba".as_bytes()))
    }
}
