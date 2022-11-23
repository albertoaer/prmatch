use std::{env, time::SystemTime};
use rand::{rngs::StdRng, SeedableRng};

mod patterns;
use patterns::Pattern;

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
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("! Usage:\n\t{0} <pattern> <seed>\n\t{0} <pattern>", args[0]);
    } else {
        let pattern = &args[1];
        let seed = match args.get(2) {
            Some(n) => get_seed_by_string(n),
            None => get_seed_by_time()
        };
        println!("- Seed: {}", seed);
        let mut rand = StdRng::seed_from_u64(seed);
        match Pattern::try_from(pattern) {
            Ok(p) => {
                let g = p.gen_one(&mut rand);
                println!("- Output: {}", g);
            },
            Err(err) => println!("! Error: {}", err),
        }
    }
}
