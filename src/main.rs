use std::{env, time::Instant};
use rand::{rngs::StdRng, SeedableRng};

mod patterns;
use patterns::{Pattern, PatternItem};


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Usage:\n\t{0} <pattern> <seed>\n\t{0} <pattern>", args[0]);
    } else {
        let pattern = &args[1];
        let seed = match args.get(2) {
            Some(n) =>
                n.bytes().map(Into::<u64>::into).reduce(|a,b| a + b).unwrap(),
            None => Instant::now().elapsed().as_micros().try_into().unwrap(),
        };
        let mut rand = StdRng::seed_from_u64(seed);
        /*match Pattern::try_from(pattern) {
            Ok(p) => {
                let g = p.gen_one(&mut rand);
                println!("Output: {}", g);
            },
            Err(err) => println!("Error: {}", err),
        }*/
        println!("Output: {}", Pattern::new(&vec![
            PatternItem::Consonant(1),
            PatternItem::Vowel(3),
            PatternItem::Digit(2),
        ]).gen_one(&mut rand));
    }
}
