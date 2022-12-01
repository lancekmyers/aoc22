use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn soln(path : &str) -> (i64, i64) {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines().flatten(); //lines(file);

    let tt = top3_cal(lines);
    
    let a = tt.max();
    let b = tt.sum();

    (a, b)
}

#[derive(Debug)]
struct Top3 {
    top3 : [i64;3],
}

impl Top3 {
    fn insert(self, x : i64) -> Self {
        let a = self.top3[0];
        let b = self.top3[1];
        let c = self.top3[2];
        if (x < a) {
            self
        } else if (x < b) {
            Top3 {top3 : [x, b, c]}
        } else if (x < c) {
            Top3 {top3 : [b, x, c]}
        } else {
            Top3 {top3 : [b, c, x]}
        }
    }
    fn sum(&self) -> i64 {
        self.top3.iter().sum()
    }
    fn max(&self) -> i64 {
        self.top3[2]
    }
    
    const EMPTY : Self = Top3 {top3: [0, 0, 0]};

}

fn top3_cal<I>(lines : I) -> Top3 
    where I : Iterator<Item = String> {

    let (tt, acc) = lines.map(
        |l| l.parse::<i64>()
    ).fold((Top3::EMPTY, 0), 
        |(tt , running_sum), x| match x {
            Ok(x) => (tt, running_sum + x),
            Err(_) => (tt.insert(running_sum), 0),
    });
    
    tt.insert(acc)
}


#[cfg(test)]
mod tests {
    use crate::day01::{soln};
    use super::Top3;

    #[test]
    fn bar() {
        let mut x = Top3::EMPTY;
        x = x.insert(6);
        x = x.insert(4);
        x = x.insert(11);
        x = x.insert(24);
        x = x.insert(10);
        assert_eq!(x.sum(), 45); 
        assert_eq!(x.max(), 24); 
    }

    #[test]
    fn given_test() {
        let (a, b) = soln("data/test01.txt");
        assert_eq!(a, 24000);
        assert_eq!(b, 45000);
    }

}