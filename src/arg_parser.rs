use structopt::StructOpt;

use crate::cache::replacement_policy::ReplacementPolicy;

#[derive(Debug, StructOpt)]
#[structopt(name = "sim_cache", about = "A homemade cache simulator in Rust")]
pub struct CacheOpts {
    #[structopt(short, long)]
    pub block_size: u64,

    #[structopt(long="l1size")]
    pub l1_size: u64,

    #[structopt(long="l1assoc")]
    pub l1_assoc: u64,

    #[structopt(long="l2size")]
    pub l2_size: u64,

    #[structopt(long="l2assoc")]
    pub l2_assoc: u64,

    #[structopt(short, long)]
    pub policy: ReplacementPolicy,

    #[structopt(short="i", long="inclusive")]
    pub is_inclusive: bool,

    #[structopt(parse(from_os_str))]
    pub trace_file: std::path::PathBuf
}