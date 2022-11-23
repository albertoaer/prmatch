use std::rc::Rc;

use crate::patterns::*;

pub fn parse_pattern(value: &String) -> Result<impl Pattern, &'static str> {
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
            "c" => CharsetPattern::Consonant,
            "v" => CharsetPattern::Vowel,
            "d" => CharsetPattern::Digit,
            _ => return Err("Unknown group item")
        }), args.1.unwrap(), args.2.unwrap()));
    }
    Ok(items)
}