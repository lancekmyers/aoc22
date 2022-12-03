use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{BinaryHeap};
use std::cmp::Reverse;

pub fn soln(path : &str) -> (i64, i64) {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines().flatten(); //lines(file);

    let tt = top3_cal(lines);
    
    let a = tt.clone().max();
    let b = tt.sum();

    (a, b)
}

#[derive(Debug, Clone)]
struct TopK<const K : usize> {
    topk : BinaryHeap<Reverse<i64>>,
}

impl<const K : usize> TopK<K> {
    fn insert(&mut self, x : i64) {
        if self.topk.len() < K {
            self.topk.push(Reverse(x));
        }
        else {
            match self.topk.peek() {
                None => 
                    self.topk.push(Reverse(x)),
                Some(Reverse(y)) => if (x > *y) {
                    let _ = self.topk.pop();
                    self.topk.push(Reverse(x));
                }
            }
        }
    }
    fn sum(self) -> i64 {
        self.topk.into_iter().map(|Reverse(x)| x).sum()
    }
    fn max(self) -> i64 {
        self.topk.into_iter().map(|Reverse(x)| x).max().unwrap_or(0)
    }
    
    fn new() -> Self { TopK { topk : BinaryHeap::new() } }

}

fn top3_cal<I>(lines : I) -> TopK<3> where I : Iterator<Item = String> {
    let mut tt = TopK::<3>::new();
    let mut acc = 0;
    for line in lines {
        let cal = line.parse::<i64>();
        match cal {
            Ok(x) => acc += x,
            Err(_) => {tt.insert(acc); acc = 0}
        }
    }
    tt.insert(acc);
    tt
}


#[cfg(test)]
mod tests {
    use crate::day01::{soln};
    use super::TopK;

    #[test]
    fn bar() {
        let mut x = TopK::<3>::new();
        x.insert(6);
        x.insert(4);
        x.insert(11);
        x.insert(24);
        x.insert(10);
        assert_eq!(x.clone().sum(), 45); 
        assert_eq!(x.max(), 24); 
    }

    #[test]
    fn given_test() {
        let (a, b) = soln("data/test01.txt");
        assert_eq!(a, 24000);
        assert_eq!(b, 45000);
    }

}