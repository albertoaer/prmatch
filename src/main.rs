use std::time::SystemTime;
use rand::{rngs::StdRng, SeedableRng};

mod parser;
use parser::Parser;

mod patterns;

mod cli;

fn get_seed_by_string(source: impl Into<String>) -> u64 {
    let src: String = source.into();
    src.bytes().enumerate()
    .map(|(i, v)| (i as u64).pow(2) * (v as u64))
    .reduce(|a,b| a + b).unwrap()
}

fn get_seed_by_time() -> u64 {
    let elapsed = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    (elapsed & Into::<u128>::into(u64::MAX)).try_into().unwrap()
}

fn main() {
    let args = cli::get_args();
    let seed = match (args.seed, args.calc_seed) {
        (_, Some(n)) => n,
        (Some(s), _) => get_seed_by_string(s),
        (None, None) => get_seed_by_time()
    };
    if !args.not_pretty {
        println!("- Seed: {}", seed);
    }
    let mut chars = args.pattern.chars();
    let mut rand = StdRng::seed_from_u64(seed);
    match Parser::new().parse_pattern(&mut chars) {
        Ok(pattern) => for _ in 0..args.n {
            if !args.not_pretty {
                print!("- Output: ")
            }
            println!("{}", pattern.gen(&mut rand));
        },
        Err(err) => println!("! Error: {}", err),
    }
}
