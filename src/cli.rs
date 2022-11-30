use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, help = "Number of iterations", default_value_t = 1)]
    pub n: u32,
    #[arg(short, long)]
    pub seed: Option<String>,
    #[arg(short, long, help = "Previously calculated seed")]
    pub calc_seed: Option<u64>,
    #[arg(long, help = "Don't prettify the output", default_value_t = false)]
    pub not_pretty: bool,
    #[arg()]
    pub pattern: String
}

pub fn get_args() -> Args {
    Args::parse()
}