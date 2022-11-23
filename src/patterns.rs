use std::rc::Rc;

use rand::{Rng, RngCore};

pub trait Pattern {
    fn gen(&self, rand: &mut dyn RngCore) -> String;
}

#[derive(Clone)]
pub struct PatternItem(pub Rc<dyn Pattern>, pub u8, pub u8);

impl Pattern for PatternItem {
    fn gen(&self, rand: &mut dyn RngCore) -> String {
        let mut ret = String::new();
        for _ in 0..rand.gen_range(self.1..=self.2) {
            ret.push_str(self.0.gen(rand).as_str());
        }
        return ret
    } 
}


#[derive(Clone, Copy)]
pub enum CharsetPattern {
    Consonant,
    Vowel,
    Digit
}

impl CharsetPattern {
    pub fn charset(&self) -> &'static [char] {
        match self {
            Self::Consonant =>
                &['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'r', 's', 't', 'p', 'q', 'v', 'w', 'x', 'y', 'z'],
            Self::Vowel =>
                &['a', 'e', 'i', 'o', 'u'],
            Self::Digit =>
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
        }
    }
}

impl Pattern for CharsetPattern {
    fn gen(&self, rand: &mut dyn RngCore) -> String {
        let charset = self.charset();
        charset[rand.gen_range(0..charset.len())].to_string()
    }
}

#[derive(Clone)]
pub struct OptionPattern(Rc<dyn Pattern>, Rc<dyn Pattern>);

impl Pattern for OptionPattern {
    fn gen(&self, rand: &mut dyn RngCore) -> String {
        if rand.gen() {
            return self.0.gen(rand)
        }
        self.1.gen(rand)
    }
}

impl Pattern for String {
    fn gen(&self, _: &mut dyn RngCore) -> String {
        self.clone()
    }
}

impl Pattern for Vec<PatternItem> {
    fn gen(&self, rand: &mut dyn RngCore) -> String {
        self.iter().map(|p| p.gen(rand))
            .reduce(|a,b| format!("{}{}", a, b)).unwrap()
    }
}