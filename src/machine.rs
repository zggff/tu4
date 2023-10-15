use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    num::ParseIntError,
    str::FromStr,
};

use super::Tape;

#[derive(Debug)]
enum Op {
    Left,
    Right,
    Set(char),
    End,
}

type Commands = HashMap<(usize, char), (Op, usize)>;

pub struct Machine {
    commands: Commands,
    tape: Tape,
}

impl Machine {
    pub fn set_input(&mut self, input: &str) {
        for c in input.chars() {
            self.tape.set_current(c);
            self.tape.shift_right();
        }
    }
    pub fn execute_with_callback<F>(&mut self, callback: F)
    where
        F: Fn(&Tape, usize, usize),
    {
        let mut state = 0;
        loop {
            let current = self.tape.get_current();
            // TODO: better handle runtime errors
            let (op, next) = self.commands.get(&(state, current)).unwrap_or_else(|| {
                panic!("No operation found for state: {state} and value: {current}")
            });
            (callback)(&self.tape, state, *next);
            match op {
                Op::End => break,
                Op::Left => self.tape.shift_left(),
                Op::Right => self.tape.shift_right(),
                Op::Set(c) => self.tape.set_current(*c),
            }
            state = *next;
        }
    }
    #[inline]
    pub fn execute(&mut self) {
        self.execute_with_callback(|_, _, _| {})
    }
    #[inline]
    pub fn display_tape(&self) {
        self.tape.display();
    }
    #[inline]
    pub fn tape(&self) -> &Tape {
        &self.tape
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseTu4Error {
    InvalidFormat,
    InvalidCellValue,
    InvalidTransition(usize),
    ParseInt(ParseIntError),
}

impl Display for ParseTu4Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTu4Error::InvalidFormat => write!(
                f,
                "the format should be: \"state,condition,operation,next\""
            ),
            ParseTu4Error::InvalidCellValue => {
                write!(f, "the cell value must be a single character")
            }
            ParseTu4Error::ParseInt(e) => write!(f, "cannot parse state: {e}"),
            ParseTu4Error::InvalidTransition(i) => write!(f, "cannot find state: {i}"),
        }
    }
}

impl std::error::Error for ParseTu4Error {}

impl FromStr for Machine {
    type Err = ParseTu4Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut commands: Commands = HashMap::new();

        let mut entrys = HashSet::new();
        let mut transitions = HashSet::new();

        for line in s.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
            let vals: Vec<&str> = line.split(',').collect();
            if vals.len() != 4 {
                return Err(ParseTu4Error::InvalidFormat);
            }

            let entry: usize = vals[0].parse().map_err(ParseTu4Error::ParseInt)?;

            if vals[1].len() != 1 {
                return Err(ParseTu4Error::InvalidCellValue);
            };
            let condition = vals[1].chars().next().unwrap();

            if vals[2].len() != 1 {
                return Err(ParseTu4Error::InvalidCellValue);
            };
            let op = vals[2].chars().next().unwrap();

            let transition: usize = vals[3].parse().map_err(ParseTu4Error::ParseInt)?;

            let op = match op {
                '<' => Op::Left,
                '>' => Op::Right,
                ' ' if entry == transition && condition == ' ' => Op::End,
                c => Op::Set(c),
            };
            entrys.insert(entry);
            transitions.insert(transition);
            commands.insert((entry, condition), (op, transition));
        }
        for transition in transitions.iter() {
            if !entrys.contains(transition) {
                return Err(ParseTu4Error::InvalidTransition(*transition));
            }
        }
        Ok(Machine {
            commands,
            tape: Tape::new(),
        })
    }
}
