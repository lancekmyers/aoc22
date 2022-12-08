use std::io::Read;

use nom::{
    self,
    bytes::complete::{take, take_while_m_n},
    character::complete::char,
    multi::{count, separated_list0},
    sequence::{pair, preceded},
    IResult, Parser,
};

use crate::aoc::Soln;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Crate {
    name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Clone)]
pub struct CrateStacks {
    stacks: Vec<Vec<Crate>>,
}

impl std::fmt::Display for CrateStacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_stack(f: &mut std::fmt::Formatter, stack: &Vec<Crate>) -> std::fmt::Result {
            let mut acc = String::new();
            acc.push_str("| ");
            for c in stack {
                acc.push_str(&c.name.to_string())
            }
            writeln!(f, "{acc}")
        }

        for stack in &self.stacks {
            fmt_stack(f, stack)?
        }
        Ok(())
    }
}

fn parse_crate(input: &str) -> nom::IResult<&str, Option<Crate>> {
    let existing_crate = nom::sequence::delimited(char('['), take(1usize), char(']'))
        .map(|x: &str| Some(Crate { name: x.to_owned() }));
    let empty_space = count(char(' '), 3).map(|_| None);

    nom::branch::alt((existing_crate, empty_space))(input)
}

fn parse_stack_line(input: &str) -> nom::IResult<&str, Vec<Option<Crate>>> {
    separated_list0(char(' '), parse_crate)(input)
}

fn parse_stacks(input: &str) -> nom::IResult<&str, CrateStacks> {
    fn build_stacks(crates: Vec<Vec<Option<Crate>>>) -> CrateStacks {
        let n = &crates.iter().nth_back(1).map_or(10, |x| x.len());

        let mut stacks: Vec<Vec<Crate>> = vec![];
        for _ in 0..*n {
            stacks.push(vec![])
        }

        for line in crates.iter().rev() {
            for (i, c) in line.iter().enumerate() {
                if let Some(c) = c {
                    stacks[i].push(c.clone())
                }
            }
        }
        CrateStacks { stacks }
    }
    separated_list0(char('\n'), parse_stack_line)(input)
        .map(|(str, crates)| (str, build_stacks(crates)))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    // move 11 from 3 to 8
    fn num(input: &str) -> IResult<&str, usize> {
        take_while_m_n(1, 20, |c: char| c.is_ascii_digit())(input.trim())
            .map(|(str, x)| (str, x.parse::<usize>().unwrap()))
    }

    let parse_count = preceded(nom::bytes::complete::tag("move"), num);
    let parse_from = preceded(nom::bytes::complete::tag(" from"), num);
    let parse_to = preceded(nom::bytes::complete::tag(" to"), num);

    let p_instr =
        pair(parse_count, pair(parse_from, parse_to)).map(|(cnt, (from, to))| Instruction {
            count: cnt,
            from,
            to,
        });

    separated_list0(char('\n'), p_instr)(input)
}

fn execute_a(instrs: &Vec<Instruction>, CrateStacks { stacks }: &mut CrateStacks) {
    for (Instruction { count, from, to }) in instrs {
        for _ in 0..*count {
            if let Some(cr) = stacks[*from - 1].pop() {
                stacks[*to - 1].push(cr);
            }
        }
    }
}

fn execute_b(instrs: &Vec<Instruction>, CrateStacks { stacks }: &mut CrateStacks) {
    for (Instruction { count, from, to }) in instrs {
        let n = stacks[*from - 1].len();
        let mut v: Vec<Crate> = stacks[*from - 1].drain((n - *count)..).collect();

        stacks[*to - 1].append(&mut v);
    }
}

pub struct Day05 {}

pub const DAY_05: Day05 = Day05 {};

impl Soln for Day05 {
    type I = (CrateStacks, Vec<Instruction>);

    type A = String;

    type B = String;

    const NAME: &'static str = "Day five";

    fn soln_a(&self, (stacks, instrs): &Self::I) -> Self::A {
        let mut stacks = stacks.clone();

        execute_a(instrs, &mut stacks);

        stacks
            .stacks
            .iter()
            .map(|st| st.last().map_or(" ", |x| &x.name))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn soln_b(&self, (stacks, instrs): &Self::I) -> Self::B {
        let mut stacks = stacks.clone();

        execute_b(instrs, &mut stacks);

        stacks
            .stacks
            .iter()
            .map(|st| st.last().map_or(" ", |x| &x.name))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn parse(&self, file: &mut std::fs::File) -> Self::I {
        let mut buf: String = String::new();
        let _ = file.read_to_string(&mut buf);

        let (crates_inp, instructions) = buf.split_once("\n\n").expect("failed to parse input");

        let (_, stacks) = parse_stacks(crates_inp).expect("failed to parse crates");
        let (_, instrs) = parse_instructions(instructions).expect("failed to parse instructions");

        (stacks, instrs)
    }
}

pub fn soln(path: &str) -> Option<(String, String)> {
    let input = std::fs::read_to_string(path).unwrap();

    let (crates_inp, instructions) = input.split_once("\n\n")?;

    let (_, stacks) = parse_stacks(crates_inp).ok()?;
    let (_, instrs) = parse_instructions(instructions).ok()?;

    let mut stacks_a = stacks.clone();
    let mut stacks_b = stacks;

    execute_a(&instrs, &mut stacks_a);
    execute_b(&instrs, &mut stacks_b);

    let a = stacks_a
        .stacks
        .iter()
        .map(|st| st.last().map_or(" ", |x| &x.name))
        .collect::<Vec<_>>()
        .join(" ");

    let b = stacks_b
        .stacks
        .iter()
        .map(|st| st.last().map_or(" ", |x| &x.name))
        .collect::<Vec<_>>()
        .join(" ");

    Some((a, b))
}

#[cfg(test)]
mod tests {
    use crate::day05::*;
    #[test]
    fn parsing() {
        let a = Some(Crate {
            name: "A".to_string(),
        });
        let b = Some(Crate {
            name: "B".to_string(),
        });
        let c = Some(Crate {
            name: "C".to_string(),
        });

        assert_eq!(Ok(("", a.clone())), parse_crate("[A]"));
        assert_eq!(Ok(("", b.clone())), parse_crate("[B]"));
        assert_eq!(Ok(("", c.clone())), parse_crate("[C]"));

        let xs = vec![a, b, c];
        assert_eq!(Ok(("", xs)), parse_stack_line("[A] [B] [C]"))
    }

    #[test]
    fn parsing_instructions() {
        let foo = Ok((
            "",
            vec![Instruction {
                count: 11,
                from: 2,
                to: 1,
            }],
        ));

        assert_eq!(foo, parse_instructions("move 11 from 2 to 1"));
    }

    #[test]
    fn parsing_input() {
        let inp = std::fs::read_to_string("data/test05.txt").unwrap();
        if let Ok((_rest, stacks)) = parse_stacks(&inp) {
            println!("{}", stacks);
        }
    }

    #[test]
    fn solution() {
        if let Some((a, b)) = soln("data/test05.txt") {
            assert_eq!(a, "C M Z");
            assert_eq!(b, "M C D");
            // println!("{}", a);
            // println!("{}", b);
        }
    }
}
