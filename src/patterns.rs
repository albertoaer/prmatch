use rand::{Rng, distributions::uniform::SampleRange};

#[derive(Clone, Copy)]
pub enum PatternItem {
    Consonant(i64, i64),
    Vowel(i64, i64),
    Digit(i64, i64)
}

pub struct Pattern {
    items: Vec<PatternItem>
}

impl PatternItem {
    pub fn charset(&self) -> &'static [char] {
        match self {
            PatternItem::Consonant(_, _) =>
                &['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'r', 's', 't', 'v', 'w', 'y', 'z'],
            PatternItem::Vowel(_, _) =>
                &['a', 'e', 'i', 'o', 'u'],
            PatternItem::Digit(_, _) =>
                &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
        }
    }

    pub fn range(&self) -> impl SampleRange<i64> {
        match *self {
            PatternItem::Consonant(min, max) => min..=max,
            PatternItem::Vowel(min, max) => min..=max,
            PatternItem::Digit(min, max) => min..=max,
        }
    }

    pub fn gen(&self, rand: &mut impl rand::RngCore) -> String {
        let mut ret = String::new();
        let charset = self.charset();
        for _ in 0..rand.gen_range(self.range()) {
            ret.push(charset[rand.gen_range(0..charset.len())]);
        }
        return ret
    }   
}

impl Pattern {
    pub fn new(items: &Vec<PatternItem>) -> Self {
        Pattern { items: items.clone() }
    }

    pub fn gen_one(&self, rand: &mut impl rand::RngCore) -> String {
        self.items.iter().map(|p| p.gen(rand)).reduce(|a,b| format!("{}{}", a, b)).unwrap()
    }
}

impl TryFrom<&String> for Pattern {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        todo!()
    }
}