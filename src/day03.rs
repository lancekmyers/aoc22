use itertools::{Itertools};


fn split_half(x : &str) -> (&str, &str) {
    x.split_at(x.len()/2)
}

fn char_to_ix(c : char) -> Option<usize> {
    let c_ = c as u8;
    let offset = if (b'A'..=b'Z').contains(&c_) {
        26
    } else {0};
    if !(b'A'..=b'z').contains(&c_) {
        None
    } else {
        c.to_digit(36).map( |i| (i - 10 + offset) as usize )
    }
    
}

struct AlphabetSet {
    set : [bool; 52]
}

impl AlphabetSet {
    fn new() -> Self {
        AlphabetSet { set : [false; 52] }
    }

    fn contains(&self, c : char) -> bool {
        if let Some(i) = char_to_ix(c) {
            self.set[i]
        } else {
            false
        }
    }

    fn insert(&mut self, c : char) {
        if let Some(i) = char_to_ix(c) {
            self.set[i] = true;
        } 
        
    }

    fn intersect(&mut self, other : Self) {
        for i in 0..52 {
           self.set[i] &= other.set[i]; 
        }
    }
}

fn mk_bs(s : &str) -> AlphabetSet {
    let mut bs = AlphabetSet::new();
    
    for c in s.chars() {
        bs.insert(c)
    }

    bs
}

fn soln_a_line(line : &str) -> char {
    let (l, r) = split_half(line);
    let bs = mk_bs(l);
    for c in r.chars() {
        if bs.contains(c) {
            return c
        }
    }
    '0'
}

fn soln_b_chunk(a : &str, b : &str, c : &str) -> Option<usize> {
    let mut s1 = mk_bs(a);
    let s2 = mk_bs(b);
    s1.intersect(s2);
    for ch in c.chars() {
        if s1.contains(ch) {
            return char_to_ix(ch)
        }
    }
    None
}

fn soln_b(str : &str) -> usize {
    let lines = str.lines();
    
    lines.chunks(3).into_iter().flat_map(
        |mut x| {
            let a = x.next()?;
            let b = x.next()?;
            let c = x.next()?;
            soln_b_chunk(a, b, c).map(|i| i + 1)
        }
    ).sum()
}


fn soln_a(str : &str) -> usize {
    str.lines()
        .map(soln_a_line)
        .flat_map(char_to_ix)
        .map(|x| x + 1).sum()
}


pub fn soln(path : &str) -> (usize, usize) {
    let contents = std::fs::read_to_string(path).unwrap();
    let a = soln_a(&contents);
    let b = soln_b(&contents);
    (a, b)
}

#[cfg(test)]
mod tests {
    use crate::day03::*;

    #[test]
    fn given() {
        assert_eq!('p', soln_a_line("vJrwpWtwJgWrhcsFMMfFFhFp"));
        let (a, b) = soln("data/test03.txt");
        assert_eq!(157, a);
        assert_eq!(70, b)
    }
    
    #[test] 
    fn foo() {
        assert_eq!(char_to_ix('p'), Some(15));
        assert_eq!(char_to_ix('L'), Some(37));
        assert_eq!(char_to_ix('P'), Some(41));
        assert_eq!(char_to_ix('v'), Some(21));
        assert_eq!(char_to_ix('t'), Some(19));
        assert_eq!(char_to_ix('s'), Some(18));
    }

}