use std::rc::Rc;

use rand::{Rng, RngCore};

pub trait PatternComponent {
    fn gen(&self, rand: &mut dyn RngCore) -> String;
}

#[derive(Clone)]
pub struct PatternItem(Rc<dyn PatternComponent>, u8, u8);

impl PatternComponent for PatternItem {
    fn gen(&self, rand: &mut dyn RngCore) -> String {
        let mut ret = String::new();
        for _ in 0..rand.gen_range(self.1..=self.2) {
            ret.push_str(self.0.gen(rand).as_str());
        }
        return ret
    } 
}


#[derive(Clone, Copy)]
pub enum BasicPatternComponent {
    Consonant,
    Vowel,
    Digit
}

impl BasicPatternComponent {
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

impl PatternComponent for BasicPatternComponent {
    fn gen(&self, rand: &mut dyn RngCore) -> String {
        let charset = self.charset();
        charset[rand.gen_range(0..charset.len())].to_string()
    }
}

pub struct Pattern {
    items: Vec<PatternItem>
}

impl Pattern {
    pub fn new(items: Vec<PatternItem>) -> Self {
        Pattern { items: items }
    }

    pub fn gen_one(&self, rand: &mut impl RngCore) -> String {
        self.items.iter().map(|p| p.gen(rand)).reduce(|a,b| format!("{}{}", a, b)).unwrap()
    }
}

impl TryFrom<&String> for Pattern {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let mut items: Vec<PatternItem> = Vec::new();
        for p in value.split('-') {
            let chars: Vec<&str> = p.split(':').collect();
            let args: (&str, Option<u8>, Option<u8>) = match chars.len() {
                1 => (chars[0], Some(1), Some(1)),
                2 => (chars[0], chars[1].parse().ok(), chars[1].parse().ok()),
                3 => (chars[0], chars[1].parse().ok(), chars[2].parse().ok()),
                _ => return Err("Group format: key-min-max or key-min or key")
            };
            if args.1 == None || args.2 == None {
                return Err("Invalid number in group range")
            }
            items.push(PatternItem(Rc::new(match args.0 {
                "c" => BasicPatternComponent::Consonant,
                "v" => BasicPatternComponent::Vowel,
                "d" => BasicPatternComponent::Digit,
                _ => return Err("Unknown group item")
            }), args.1.unwrap(), args.2.unwrap()));
        }
        Ok(Pattern::new(items))
    }
}