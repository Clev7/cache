mod cache;

use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
enum ReplacementPolicy {
    Lru     = 0,
    Plru    = 1,
    Optimal = 2
}

impl FromStr for ReplacementPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "0" | "lru" => Ok(Self::Lru),
            "1" | "plru" => Ok(Self::Plru),
            "2" | "optimal" => Ok(Self::Optimal),
            _ => Err(format!("Invalid policy {}: Use 0 (LRU), 1 (PLRU), or 2 (Optimal)", s))
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "sim_cache", about = "A homemade cache simulator in Rust")]
pub struct CacheOpts {
    #[structopt(short, long)]
    block_size: u64,

    #[structopt(long="l1size")]
    l1_size: u64,

    #[structopt(long="l1assoc")]
    l1_assoc: u64,

    #[structopt(long="l2size")]
    l2_size: u64,

    #[structopt(long="l2assoc")]
    l2_assoc: u64,

    #[structopt(short, long)]
    policy: ReplacementPolicy,

    #[structopt(short="i", long="inclusive")]
    is_inclusive: bool,

    #[structopt(parse(from_os_str))]
    trace_file: std::path::PathBuf
}