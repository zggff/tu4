use std::{collections::LinkedList, iter};

pub struct Tape {
    left: LinkedList<char>,
    current: char,
    right: LinkedList<char>,
}

impl Tape {
    pub fn new() -> Self {
        Tape {
            left: LinkedList::new(),
            current: ' ',
            right: LinkedList::new(),
        }
    }
    pub fn shift_left(&mut self) {
        let new_current = self.left.pop_front().unwrap_or(' ');
        self.right.push_front(self.current);
        self.current = new_current;
    }
    pub fn shift_right(&mut self) {
        let new_current = self.right.pop_front().unwrap_or(' ');
        self.left.push_front(self.current);
        self.current = new_current;
    }
    pub fn get_current(&mut self) -> char {
        self.current
    }
    pub fn set_current(&mut self, val: char) {
        self.current = val;
    }
    pub fn display(&self) {
        let show: String = iter::repeat(' ')
            .take(self.left.len())
            .chain(iter::once('^'))
            .chain(iter::repeat(' ').take(self.right.len()))
            .collect();
        println!("{}", self.to_string());
        println!("{}", show)
    }
}

impl ToString for Tape {
    fn to_string(&self) -> String {
        self.left
            .iter()
            .rev()
            .chain(iter::once(&self.current))
            .chain(self.right.iter())
            .map(|&c| if c.is_whitespace() { '_' } else { c })
            .collect()
    }
}

impl Default for Tape {
    fn default() -> Self {
        Self::new()
    }
}
