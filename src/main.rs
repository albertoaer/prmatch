use std::env;

use rand::{rngs::StdRng, SeedableRng};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <pattern> <seed>", args[0]);
    } else {
        let pattern = &args[1];
        let seed = &args[2];
        let mut rand = StdRng::seed_from_u64(
            seed.bytes().map(Into::<u64>::into).reduce(|a,b| a + b).unwrap()
        );
    }
}
