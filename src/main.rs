use cache::CacheOpts;
use structopt::StructOpt;

fn main() {
    let args = CacheOpts::from_args();

    println!("{:?}", args);
}