use rand::Rng;

#[derive(Clone, Copy)]
pub enum PatternItem {
    Consonant(i64),
    Vowel(i64),
    Digit(i64)
}

pub struct Pattern {
    items: Vec<PatternItem>
}

fn gen_many(max: i64, rand: &mut impl rand::RngCore, charset: Vec<char>) -> String {
    let mut ret = String::new();
    for _ in 0..max {
        ret.push(charset[rand.gen_range(0..charset.len())]);
    }
    return ret
}

impl Pattern {
    pub fn new(items: &Vec<PatternItem>) -> Self {
        Pattern { items: items.clone() }
    }

    pub fn gen_one(&self, rand: &mut impl rand::RngCore) -> String {
        self.items.iter().map(|p| match p {
            PatternItem::Consonant(n) =>
                gen_many(*n, rand, vec!['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'r', 's', 't', 'v', 'w', 'y', 'z']),
            PatternItem::Vowel(n) =>
                gen_many(*n, rand, vec!['a', 'e', 'i', 'o', 'u']),
            PatternItem::Digit(n) =>
                gen_many(*n, rand, ('0'..='9').collect()),
        }).reduce(|a,b| format!("{}{}", a, b)).unwrap()
    }
}

impl TryFrom<&String> for Pattern {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        todo!()
    }
}