use std::{rc::Rc, str::Chars};

use crate::patterns::*;

#[derive(Clone)]
enum ParserStep {
    Empty,
    BeginRange(Rc<dyn Pattern>),
    Range(Rc<dyn Pattern>, Vec<char>),
    RangeClose(Rc<dyn Pattern>, Vec<char>, Vec<char>)
}

fn vec_char_to_number(n: &Vec<char>) -> Option<u32> {
    n.into_iter().enumerate()
        .map(|(i, c)| (*c as u32 - '0' as u32) * 10_u32.pow((n.len() - i - 1) as u32))
        .reduce(|a,b| a + b)
}

impl ParserStep {
    fn get_pattern_item(&self) -> Result<PatternItem, String> {
        match self {
            Self::Empty => Err("There is no group".to_string()),
            Self::BeginRange(target) => Ok(PatternItem(target.clone(), 1, 1)),
            Self::Range(target, max) =>
            if let Some(bound) = vec_char_to_number(max) {
                Ok(PatternItem(target.clone(), bound, bound))
            } else {
                Err("Empty range value".to_string())
            }
            Self::RangeClose(target, min, max) =>
            if let (Some(min), Some(max)) = (vec_char_to_number(min), vec_char_to_number(max)) {
                Ok(PatternItem(target.clone(), min, max))
            } else {
                Err("Empty range value".to_string())
            }
        }
    }
}

pub struct Parser {
    stack: Vec<Vec<PatternItem>>,
    items: Vec<PatternItem>,
    step: ParserStep
}

impl Parser {
    pub fn new() -> Self {
        Parser { stack: Vec::new(), items: Vec::new(), step: ParserStep::Empty }
    }

    fn open_group(&mut self) {
        self.stack.push(self.items.clone());
        self.items = Vec::new();
        self.step = ParserStep::Empty;
    }

    fn close_group(&mut self) -> Result<(), String> {
        match self.stack.pop() {
            Some(v) => {
                self.step = ParserStep::BeginRange(Rc::new(self.items.clone()));
                self.items = v;
            },
            None => return Err("Unexpected group close with no openned group".to_string()),
        }
        Ok(())
    }

    fn must_push_item(&mut self) -> Result<(), String> {
        self.items.push(self.step.get_pattern_item()?);
        self.step = ParserStep::Empty;
        Ok(())
    }

    fn parse_char(&mut self, c: char) -> Result<(), String> {
        if c == ')' {
            self.must_push_item()?;
            self.close_group()?;
            return Ok(())
        } else if c == '-' {
            self.must_push_item()?;
            return Ok(())
        }
        match &mut self.step {
            ParserStep::Empty => match c {
                'c' => self.step = ParserStep::BeginRange(Rc::new(CharsetPattern::Consonant)),
                'v' => self.step = ParserStep::BeginRange(Rc::new(CharsetPattern::Vowel)),
                'd' => self.step = ParserStep::BeginRange(Rc::new(CharsetPattern::Digit)),
                '(' => self.open_group(),
                _ => return Err(format!("Unexpected charset: {}", c))
            }
            ParserStep::BeginRange(i) => match c {
                ':' => self.step = ParserStep::Range(i.clone(), Vec::new()),
                _ => return Err(format!("Unexpected token: {}", c))
            }
            ParserStep::Range(i, r1) => match c {
                '0'..='9' => r1.push(c),
                ':' => self.step = ParserStep::RangeClose(i.clone(), r1.clone(), Vec::new()),
                _ => return Err(format!("Unexpected token: {}", c))
            }
            ParserStep::RangeClose(_, _, r2) => match c {
                '0'..='9' => r2.push(c),
                _ => return Err(format!("Unexpected token: {}", c))
            }
        }
        Ok(())
    }

    pub fn parse_pattern(&mut self, chars: &mut Chars) -> Result<impl Pattern, String> {
        for c in chars {
            self.parse_char(c)?
        }
        self.must_push_item()?;
        if self.stack.len() > 0 {
            return Err("Unclosed group".to_string())
        }
        Ok(self.items.clone())
    }
}