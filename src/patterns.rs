use rand::{Rng, distributions::uniform::SampleRange};

#[derive(Clone, Copy)]
pub enum PatternItem {
    Consonant(i64, i64),
    Vowel(i64, i64),
    Digit(i64, i64)
}

impl PatternItem {
    pub fn charset(&self) -> &'static [char] {
        match self {
            PatternItem::Consonant(_, _) =>
                &['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'r', 's', 't', 'p', 'q', 'v', 'w', 'x', 'y', 'z'],
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

pub struct Pattern {
    items: Vec<PatternItem>
}

impl Pattern {
    pub fn new(items: Vec<PatternItem>) -> Self {
        Pattern { items: items }
    }

    pub fn gen_one(&self, rand: &mut impl rand::RngCore) -> String {
        self.items.iter().map(|p| p.gen(rand)).reduce(|a,b| format!("{}{}", a, b)).unwrap()
    }
}

impl TryFrom<&String> for Pattern {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let mut items = Vec::new();
        for p in value.split('-') {
            let chars: Vec<&str> = p.split(':').collect();
            let args: (&str, Option<i64>, Option<i64>) = match chars.len() {
                1 => (chars[0], Some(1), Some(1)),
                2 => (chars[0], chars[1].parse().ok(), chars[1].parse().ok()),
                3 => (chars[0], chars[1].parse().ok(), chars[2].parse().ok()),
                _ => return Err("Group format: key-min-max or key-min or key")
            };
            if args.1 == None || args.2 == None {
                return Err("Invalid number in group range")
            }
            items.push(match args.0 {
                "c" => PatternItem::Consonant(args.1.unwrap(), args.2.unwrap()),
                "v" => PatternItem::Vowel(args.1.unwrap(), args.2.unwrap()),
                "d" => PatternItem::Digit(args.1.unwrap(), args.2.unwrap()),
                _ => return Err("Unknown group item")
            });
        }
        Ok(Pattern::new(items))
    }
}