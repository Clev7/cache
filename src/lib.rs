mod cache;

use structopt::StructOpt;

#[derive(Debug)]
enum ReplacementPolicy {
    Lru     = 0,
    Plru    = 1,
    Optimal = 2
}

#[derive(Debug, StructOpt)]
#[structopt(name = "sim_cache", about = "A homemade cache simulator in Rust")]
struct CacheOpts {
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

    // // #[structopt(short, long)]
    // policy: ReplacementPolicy,

    #[structopt(short="i", long="inclusive")]
    is_inclusive: bool,

    #[structopt(parse(from_os_str))]
    trace_file: std::path::PathBuf
}

pub fn get_args() {
    return CacheOpts::from_args();
}
