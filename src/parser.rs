use std::{rc::Rc, str::Chars};

use crate::patterns::*;

#[derive(Clone)]
enum ParserStep {
    Empty,
    BasicWrap(Rc<dyn Pattern>),
    Range(Rc<dyn Pattern>, Vec<char>),
    RangeClose(Rc<dyn Pattern>, Vec<char>, Vec<char>),
    Raw(String)
}

fn vec_char_to_number(n: &Vec<char>) -> Option<u32> {
    n.into_iter().enumerate()
        .map(|(i, c)| (*c as u32 - '0' as u32) * 10_u32.pow((n.len() - i - 1) as u32))
        .reduce(|a,b| a + b)
}

impl ParserStep {
    fn get_pattern_item(&self) -> Result<Rc<dyn Pattern>, String> {
        match self {
            Self::Empty => Err("There is no group".to_string()),
            Self::BasicWrap(target) => Ok(Rc::new(PatternItem(target.clone(), 1, 1))),
            Self::Range(target, max) =>
                if let Some(bound) = vec_char_to_number(max) {
                    Ok(Rc::new(PatternItem(target.clone(), bound, bound)))
                } else {
                    Err("Empty range value".to_string())
                }
            Self::RangeClose(target, min, max) =>
                if let (Some(min), Some(max)) = (vec_char_to_number(min), vec_char_to_number(max)) {
                    if min > max {
                        return Err("Invalid range".to_string())
                    }
                    Ok(Rc::new(PatternItem(target.clone(), min, max)))
                } else {
                    Err("Empty range value".to_string())
                }
            Self::Raw(s) =>
                if s.is_empty() {
                    Err("Empty sequence".to_string())
                } else {
                    Ok(Rc::new(s.clone()))
                }
        }
    }
}

use ParserStep::*;

#[derive(Clone, Copy, PartialEq)]
enum ParserGroupMode {
    Concat,
    Option
}

#[derive(Clone)]
struct ParserGroup {
    items: Vec<Rc<dyn Pattern>>,
    mode: ParserGroupMode
}

impl ParserGroup {
    fn new(mode: ParserGroupMode) -> Self {
        ParserGroup { items: Vec::new(), mode: mode }
    }

    fn push(&mut self, item: Rc<dyn Pattern>) {
        self.items.push(item)
    }

    fn get_pattern_item(&self) -> Result<Rc<dyn Pattern>, String> {
        if self.items.is_empty() {
            return Err("Group is empty".to_string())
        }
        Ok(match self.mode {
            ParserGroupMode::Concat => Rc::new(self.items.clone()),
            ParserGroupMode::Option => Rc::new(OptionPattern(self.items.clone()))
        })
    }
}

pub struct Parser {
    stack: Vec<ParserGroup>,
    items: ParserGroup,
    step: ParserStep
}

impl Parser {

    pub fn new() -> Self {
        Parser { stack: Vec::new(), items: ParserGroup::new(ParserGroupMode::Concat), step: Empty }
    }

    fn open_group(&mut self, mode: ParserGroupMode) {
        self.stack.push(self.items.clone());
        self.items = ParserGroup::new(mode);
        self.step = Empty;
    }

    fn close_group(&mut self, mode: ParserGroupMode) -> Result<(), String> {
        match self.stack.pop() {
            Some(v) => {
                if self.items.mode != mode {
                    return Err("Closing wrong group".to_string())
                }
                self.step = BasicWrap(self.items.get_pattern_item()?);
                self.items = v;
            },
            None => return Err("Unexpected group close with no openned group".to_string()),
        }
        Ok(())
    }

    fn must_push_item(&mut self) -> Result<(), String> {
        self.items.push(self.step.get_pattern_item()?);
        self.step = Empty;
        Ok(())
    }

    fn parse_char(&mut self, c: char) -> Result<(), String> {
        match (c, &self.step) {
            (
                '[' | '{' | '%' | 'c' | 'v' | 'd' | 's',
                BasicWrap(_) | Range(_, _) | RangeClose(_, _, _)
            ) => self.must_push_item()?,
            _ => ()
        }
        match (c, &mut self.step) {
            ('[', Empty) => self.open_group(ParserGroupMode::Concat),
            (']', _) => {
                self.must_push_item()?;
                self.close_group(ParserGroupMode::Concat)?;
            },
            ('{', Empty) => self.open_group(ParserGroupMode::Option),
            ('}', _) => {
                self.must_push_item()?;
                self.close_group(ParserGroupMode::Option)?;
            },
            ('#', _) => self.step = ParserStep::BasicWrap(Rc::new(SubsetPattern(self.step.get_pattern_item()?))),
            ('%', Empty) => self.step = Raw(String::new()),
            ('c', Empty) => self.step = BasicWrap(Rc::new(CharsetPattern::Consonant)),
            ('v', Empty) => self.step = BasicWrap(Rc::new(CharsetPattern::Vowel)),
            ('d', Empty) => self.step = BasicWrap(Rc::new(CharsetPattern::Digit)),
            ('s', Empty) => self.step = BasicWrap(Rc::new(String::from(" "))),
            (':', BasicWrap(i)) => self.step = Range(i.clone(), Vec::new()),
            (':', Range(i, r1)) => self.step = RangeClose(i.clone(), r1.clone(), Vec::new()),
            ('0'..='9', Range(_, r1)) => r1.push(c),
            ('0'..='9', RangeClose(_, _, r2)) => r2.push(c),
            (_, Raw(r)) => r.push(c),
            (_, _) => return Err(format!("Unexpected token: {}", c))
        }
        Ok(())
    }

    pub fn parse_pattern(&mut self, chars: &mut Chars) -> Result<Rc<dyn Pattern>, String> {
        for c in chars {
            self.parse_char(c)?
        }
        self.must_push_item()?;
        if self.stack.len() > 0 {
            return Err("Unclosed group".to_string())
        }
        Ok(self.items.get_pattern_item()?)
    }
}